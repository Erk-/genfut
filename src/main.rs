#![allow(unused_must_use)]
#![allow(unused_variables)]

use std::fs::create_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;

mod arrays;
mod entry;
mod genc;
use crate::arrays::gen_impl_futhark_types;
use crate::entry::*;
use crate::genc::*;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "genfut",
    about = "Generates rust code to interface with generated futhark code."
)]
struct Opt {
    /// Output dir
    #[structopt(name = "NAME")]
    name: String,

    /// File to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let futhark_file = opt.file;
    let out_dir_str = format!("./{}", opt.name);
    let out_dir = Path::new(&out_dir_str);

    // Create dir
    if let Err(e) = create_dir(out_dir) {
        println!("Error creating dir ({})", e);
    }

    // Generate C code, Though only headerfiles are needed.
    // In general C files are generated when build at the user.
    gen_c(&futhark_file, &out_dir);

    // copy futhark file
    if let Err(e) = std::fs::copy(futhark_file, PathBuf::from(out_dir).join("lib/a.fut")) {
        println!("Error copying file: {}", e);
    }

    // Generate bindings
    let src_dir = PathBuf::from(out_dir).join("src");
    if let Err(e) = create_dir(&src_dir) {
        println!("Error creating dir {:#?}, ({})", src_dir, e);
    }

    generate_bindings(
        &PathBuf::from(out_dir).join("lib/a.h"),
        &PathBuf::from(out_dir).join("src"),
    );

    let headers = std::fs::read_to_string(PathBuf::from(out_dir).join("lib/a.h"))
        .expect("Could not read headers");

    let re_array_types = Regex::new(r"struct (futhark_.+_\d+d) ;").expect("Regex failed!");
    let array_types: Vec<String> = re_array_types
        .captures_iter(&headers)
        .map(|c| c[1].to_owned())
        .collect();
    //println!("{:#?}", array_types);
    //println!("{}", gen_impl_futhark_types(&array_types));
    // STATIC FILES
    // build.rs
    let static_build = include_str!("static/build.rs");
    let mut build_file =
        File::create(PathBuf::from(out_dir).join("build.rs")).expect("File creation failed!");
    write!(&mut build_file, "{}", static_build);

    // Cargo.toml
    let static_cargo = format!(include_str!("static/static_cargo.toml"), libname = opt.name);
    let mut cargo_file =
        File::create(PathBuf::from(out_dir).join("Cargo.toml")).expect("File creation failed!");
    write!(&mut cargo_file, "{}", static_cargo);

    // src/context.rs
    let static_context = include_str!("static/static_context.rs");
    let mut context_file =
        File::create(PathBuf::from(out_dir).join("src/context.rs")).expect("File creation failed!");
    writeln!(&mut context_file, "{}", static_context);

    // src/traits.rs
    let static_traits = include_str!("static/static_traits.rs");
    let mut traits_file =
        File::create(PathBuf::from(out_dir).join("src/traits.rs")).expect("File creation failed!");
    writeln!(&mut traits_file, "{}", static_traits);

    let static_array = include_str!("static/static_array.rs");

    let mut array_file =
        File::create(PathBuf::from(out_dir).join("src/arrays.rs")).expect("File creation failed!");
    writeln!(&mut array_file, "{}", static_array);
    writeln!(&mut array_file, "{}", gen_impl_futhark_types(&array_types));

    let re_entry_points = Regex::new(r"(?m)int futhark_entry_(.+)\(struct futhark_context \*ctx,(\s*(:?const\s*)?(:?struct\s*)?[a-z0-9_]+\s\**[a-z0-9]+,?\s?)+\);").unwrap();

    let entry_points: Vec<String> = re_entry_points
        .captures_iter(&headers)
        .map(|c| c[0].to_owned())
        .collect();
    let static_lib = include_str!("static/static_lib.rs");
    let mut methods_file =
        File::create(PathBuf::from(out_dir).join("src/lib.rs")).expect("File creation failed!");
    writeln!(&mut methods_file, "{}", static_lib);
    writeln!(&mut methods_file, "{}", gen_entry_points(&entry_points));
    //println!("{:#?}", entry_points);
}
