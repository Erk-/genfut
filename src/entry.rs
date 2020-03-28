use std::fmt::Write;

//extern crate inflector;
use inflector::Inflector;
use regex::Regex;

fn type_translation(input: String) -> String {
    if input.starts_with("futhark") {
        format!("{}", auto_ctor(&input))
    } else {
        let mut buffer = String::new();
        if input.starts_with("int8") {
            write!(&mut buffer, "i8");
        } else if input.starts_with("int") {
            write!(&mut buffer, "i{}", &input[3..5]);
        } else if input.starts_with("uint8") {
            write!(&mut buffer, "u8");
        } else if input.starts_with("uint") {
            write!(&mut buffer, "u{}", &input[4..6]);
        } else if input.starts_with("float") {
            write!(&mut buffer, "f32");
        } else if input.starts_with("double") {
            write!(&mut buffer, "f64");
        }
        buffer
    }
}

fn ctor_array_type(t: &str, dim: usize) -> String {
    format!("Array_{}_{}d", t, dim)
}

fn parse_array_type(t: &str) -> Option<(String, usize)> {
    let re_array_type = Regex::new(r"futhark_(.+)_(\d+)d").unwrap();
    if let Some(captures) = re_array_type.captures(t) {
        let dim: usize = captures[2].parse().unwrap();
        let ftype = &captures[1];
        Some((ftype.to_string(), dim))
    } else {
        None
    }
}
fn auto_ctor(t: &str) -> String {
    let re_array_type = Regex::new(r"futhark_(.+)_(\d+)d").unwrap();
    if let Some((ftype, dim)) = parse_array_type(t) {
        ctor_array_type(&ftype, dim)
    } else {
        to_opaque_type_name(t)
    }
}

pub(crate) fn gen_entry_point(input: &str) -> (String, String, Vec<String>) {
    let re_name = Regex::new(r"futhark_entry_(.+)\(").unwrap();
    let re_arg_pairs =
        Regex::new(r"(?m)\s*(?:const\s*)?(?:struct\s*)?([a-z0-9_]+)\s\**([a-z0-9]+),?\s?").unwrap();

    let arg_pairs: Vec<(String, String)> = re_arg_pairs
        .captures_iter(input)
        .skip(2)
        .map(|c| (c[1].to_owned(), c[2].to_owned()))
        .collect();
    let name = re_name.captures(input).unwrap()[1].to_owned();
    let mut buffer = format!("pub fn {name}", name = name);

    write!(&mut buffer, "(&mut self, ");
    for (i, (argtype, argname)) in arg_pairs.iter().enumerate() {
        if argname.starts_with("in") {
            write!(
                &mut buffer,
                "{}: {}, ",
                argname,
                type_translation(String::from(argtype.clone()))
            );
        }
    }
    write!(&mut buffer, ") -> ");
    let mut output_buffer = String::from("Result<(");
    let mut output_counter = 0;
    for (i, (argtype, argname)) in arg_pairs.iter().enumerate() {
        if argname.starts_with("out") {
            if output_counter > 0 {
                write!(&mut output_buffer, ", ");
            }
            output_counter += 1;
            write!(
                &mut output_buffer,
                "{}",
                type_translation(String::from(argtype.clone()))
            );
        }
    }
    write!(&mut output_buffer, ")>");
    writeln!(&mut buffer, "{}", output_buffer);

    write!(
        &mut buffer,
        "{{\nlet ctx = self.ptr();\nunsafe{{\n_{name}(ctx, ",
        name = name
    );
    for (i, (argtype, argname)) in arg_pairs.iter().enumerate() {
        if argname.starts_with("in") {
            if argtype.starts_with("futhark") {
                write!(&mut buffer, "{}.as_raw_mut(), ", argname);
            } else {
                write!(&mut buffer, "{}, ", argname);
            }
        }
    }
    write!(&mut buffer, ")\n}}}}\n");

    // END OF FIRST PART
    let mut buffer2 = String::new();
    write!(
        &mut buffer2,
        "unsafe fn _{name}(ctx: *mut bindings::futhark_context, ",
        name = name
    );
    for (i, (argtype, argname)) in arg_pairs.iter().enumerate() {
        if argname.starts_with("in") {
            if argtype.starts_with("futhark") {
                write!(&mut buffer2, "{}: *const bindings::{}, ", argname, argtype);
            } else {
                write!(
                    &mut buffer2,
                    "{}: {}, ",
                    argname,
                    type_translation(String::from(argtype.clone()))
                );
            }
        }
    }
    writeln!(&mut buffer2, ") -> {} {{", output_buffer);
    for (i, (argtype, argname)) in arg_pairs.iter().enumerate() {
        if argname.starts_with("out") {
            if argtype.starts_with("futhark") {
                writeln!(
                    &mut buffer2,
                    "let mut raw_{} = std::ptr::null_mut();",
                    argname
                );
            } else {
                writeln!(
                    &mut buffer2,
                    "let mut raw_{} = {}::default();",
                    argname,
                    type_translation(String::from(argtype.clone()))
                );
            }
        }
    }

    write!(
        &mut buffer2,
        "\nif bindings::futhark_entry_{name}(ctx, ",
        name = name
    );
    for (i, (argtype, argname)) in arg_pairs.iter().enumerate() {
        if argname.starts_with("out") {
            write!(&mut buffer2, "&mut raw_{}, ", argname);
        }
    }
    for (i, (argtype, argname)) in arg_pairs.iter().enumerate() {
        if argname.starts_with("in") {
            write!(&mut buffer2, "{}, ", argname);
        }
    }
    writeln!(
        &mut buffer2,
        ") != 0 {{
return Err(FutharkError::new(ctx).into());}}"
    );

    let mut opaque_types = Vec::new();
    // OUTPUT
    let mut result_counter = 0;
    write!(&mut buffer2, "Ok(");
    for (i, (argtype, argname)) in arg_pairs.iter().enumerate() {
        if argname.starts_with("out") {
            dbg!(argtype);
            if !parse_array_type(argtype).is_some() {
                dbg!("opaque type", &argtype);
                opaque_types.push(argtype.clone());
            }
            if result_counter > 0 {
                write!(&mut buffer2, ", ");
            }
            result_counter += 1;
            if argtype.starts_with("futhark") {
                writeln!(
                    &mut buffer2,
                    "{}::from_ptr(ctx, raw_{})",
                    auto_ctor(&argtype),
                    argname
                );
            } else {
                writeln!(&mut buffer2, "raw_{}", argname);
            }
        }
    }
    write!(&mut buffer2, ")\n}}");

    dbg!(&opaque_types);

    (buffer, buffer2, opaque_types)
}
fn to_opaque_type_name(s: &str) -> String {
    let mut rust_opaque_type = s.to_camel_case();

    if let Some(r) = rust_opaque_type.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    rust_opaque_type
}

fn gen_opaque_type(opaque_type: &str) -> String {
    let rust_opaque_type = to_opaque_type_name(opaque_type);
    let base_type = if opaque_type.starts_with("futhark_") {
        &opaque_type[8..]
    } else {
        panic!("Apparent opaque type didn't start with futhark_.")
    };
    format!(
        include_str!("static/static_opaque_types.rs"),
        opaque_type = rust_opaque_type,
        futhark_type = format!("bindings::{}", opaque_type),
        base_type = base_type
    )
}

pub(crate) fn gen_entry_points(input: &Vec<String>) -> String {
    let mut buffer = String::from(
        r#"impl FutharkContext {
"#,
    );
    let mut opaque_types = Vec::new();
    let mut buffer2 = String::new();
    for t in input {
        let (a, b, otypes) = gen_entry_point(&t);
        opaque_types.extend(otypes);
        writeln!(&mut buffer, "{}", a).expect("Write failed!");
        writeln!(&mut buffer2, "{}", b).expect("Write failed!");
    }

    opaque_types.sort();
    opaque_types.dedup();
    dbg!(&opaque_types);
    for (i, opaque_type) in opaque_types.iter().enumerate() {
        if i > 0 {
            write!(&mut buffer2, ", ");
        }
        if opaque_type.starts_with("futhark") {
            writeln!(&mut buffer2, "{}", gen_opaque_type(opaque_type));
        }
    }

    writeln!(&mut buffer, "}}").expect("Write failed!");
    writeln!(&mut buffer, "{}", buffer2).expect("Write failed!");

    buffer
}
