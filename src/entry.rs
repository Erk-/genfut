use std::fmt::Write;

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

fn auto_ctor(t: &str) -> String {
    let re_array_type = Regex::new(r"futhark_(.+)_(\d+)d").unwrap();
    let captures = re_array_type.captures(t).unwrap();
    let dim: usize = captures[2].parse().unwrap();
    let ftype = &captures[1];
    ctor_array_type(ftype, dim)
}


pub(crate) fn gen_entry_point(input: &str) -> (String, String) {
    let re_name = Regex::new(r"futhark_entry_(.+)\(").unwrap();
    let re_entry_points = Regex::new(r"(?m)\s*(?:const\s*)?(?:struct\s*)?([a-z0-9_]+)\s\**([a-z0-9]+),?\s?").unwrap();
    let entry_points: Vec<(String, String)> = re_entry_points
        .captures_iter(input).skip(2)
        .map(|c| (c[1].to_owned(), c[2].to_owned()))
        .collect();
    let name = re_name.captures(input).unwrap()[1].to_owned();
    //println!("input: {}\nname: {}\n{:#?}", input, name, entry_points);
    let mut buffer = format!("pub fn {name}", name=name);
    // for (i, e) in entry_points.iter().enumerate() {
    //     if e.1.starts_with("in") {
    //         write!(&mut buffer, "T{}, ", i);
    //     }
    // }
    write!(&mut buffer, "(&mut self, ");
    for (i, e) in entry_points.iter().enumerate() {
        if e.1.starts_with("in") {
            // if e.0.starts_with("futhark") {
            //     write!(&mut buffer, "{}: T{}, ", e.1, i);
            // } else {
            write!(&mut buffer, "{}: {}, ",
                   e.1,
                   type_translation(String::from(e.0.clone())));
            // }
        } 
    }
    write!(&mut buffer, ") -> ");
    let mut output_buffer = String::from("(");
    let mut output_counter = 0;
    for (i, e) in entry_points.iter().enumerate() {
        if e.1.starts_with("out") {
            if output_counter > 0 {
                write!(&mut output_buffer, ", ");
            }
            output_counter += 1;
            write!(&mut output_buffer, "{}", type_translation(String::from(e.0.clone())));
        }
    }
    write!(&mut output_buffer, ")");
    writeln!(&mut buffer, "{}", output_buffer);
    // for (i, e) in entry_points.iter().enumerate() {
    //     if e.1.starts_with("in") && e.0.starts_with("futhark") {
    //         write!(&mut buffer, "T{}: IntoCtx<futhark_{}>,\n", i, auto_ctor(&e.0));
    //     }
    // }
    write!(&mut buffer, "{{\nlet ctx = self.ptr();\nunsafe{{\n_{name}(ctx, ", name=name);
    for (i, e) in entry_points.iter().enumerate() {
        if e.1.starts_with("in") {
            if e.0.starts_with("futhark") {
                write!(&mut buffer, "{}.as_raw_mut(), ", e.1);
            } else {
                write!(&mut buffer, "{}, ", e.1);
            }
        }
    }    
    write!(&mut buffer, ")\n}}}}\n");

    // END OF FIRST PART
    let mut buffer2 = String::new();
    write!(&mut buffer2, "unsafe fn _{name}(ctx: *mut bindings::futhark_context, ", name=name);
    for (i, e) in entry_points.iter().enumerate() {
        if e.1.starts_with("in") {
            if e.0.starts_with("futhark") {
                write!(&mut buffer2, "{}: *const bindings::{}, ", e.1, e.0);
            } else {
                write!(&mut buffer2, "{}: {}, ", e.1, type_translation(String::from(e.0.clone())));
            }
        }
    } 
    writeln!(&mut buffer2, ") -> {} {{", output_buffer);
    for (i, e) in entry_points.iter().enumerate() {
        if e.1.starts_with("out") {
            if e.0.starts_with("futhark") {
                writeln!(&mut buffer2, "let mut raw_{} = std::ptr::null_mut();", e.1);
            } else {
                writeln!(&mut buffer2, "let mut raw_{} = {}::default();", e.1,
                       type_translation(String::from(e.0.clone())));
            }
        }
    }

    write!(&mut buffer2, "\nbindings::futhark_entry_{name}(ctx, ", name=name);
    for (i, e) in entry_points.iter().enumerate() {
        if e.1.starts_with("out") {
            write!(&mut buffer2, "&mut raw_{}, ", e.1);
        }
    }
    for (i, e) in entry_points.iter().enumerate() {
        if e.1.starts_with("in") {
            write!(&mut buffer2, "{}, ", e.1);
        }
    }
    writeln!(&mut buffer2, ");");

    // FREE
    for (i, e) in entry_points.iter().enumerate() {
        if e.1.starts_with("in") && e.0.starts_with("futhark") {
            write!(&mut buffer2, "bindings::{}::free(ctx, {} as *mut bindings::{});\n", e.0, e.1, e.0);
        }
    }

    // OUTPUT
    let mut result_counter = 0;
    write!(&mut buffer2, "(");
    for (i, e) in entry_points.iter().enumerate() {
        if e.1.starts_with("out") {
            if result_counter > 0 {
                write!(&mut buffer2, ", ");
            }
            result_counter += 1;
            if e.0.starts_with("futhark") {
                writeln!(&mut buffer2, "{}::from_ptr(ctx, raw_{})", auto_ctor(&e.0), e.1);
            } else {
                writeln!(&mut buffer2, "raw_{}", e.1);
            }
        }
    }
    write!(&mut buffer2, ")\n}}");
    (buffer, buffer2)
}

pub(crate) fn gen_entry_points(input: &Vec<String>) -> String {
    let mut buffer = String::from(r#"impl FutharkContext {
"#);
    let mut buffer2 = String::new();
    for t in input {
        let (a, b) = gen_entry_point(&t);
        writeln!(&mut buffer, "{}", a).expect("Write failed!");
        writeln!(&mut buffer2, "{}", b).expect("Write failed!");
    }
    writeln!(&mut buffer, "}}").expect("Write failed!");
    writeln!(&mut buffer, "{}", buffer2).expect("Write failed!");
    buffer
}
