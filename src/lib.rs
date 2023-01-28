//!# Genfut
//!
//!This is a tool to generate a Rust library to interact with exported functions from a Futhark file.
//!
//!## Usage
//!
//!### As an executable binary
//!```shell
//!genfut <Rust lib name> <futhark_file.fut>
//!```
//!
//!### As a library
//!
//!`build.rs`
//!```rust, no_run
//! use genfut::{Opt, genfut};
//!
//! genfut(Opt {
//!     name: "<Rust lib name>".to_string(),
//!     file: std::path::PathBuf::from("futhark_file.fut"),
//!     author: "Name <name@example.com>".to_string(),
//!     version: "0.1.0".to_string(),
//!     license: "YOLO".to_string(),
//!     description: "Futhark example".to_string(),
//! })
//!```
#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(clippy::uninlined_format_args)]

use clap::Parser;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use regex::Regex;

mod arrays;
mod entry;
mod genc;
use crate::arrays::gen_impl_futhark_types;
use crate::entry::*;
pub use crate::genc::Backend;
use crate::genc::{gen_c, generate_bindings};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Options {
    /// Output dir
    #[arg(name = "NAME")]
    pub name: String,

    /// File to process
    #[arg(name = "FILE")]
    pub file: PathBuf,

    /// License
    #[arg(long, name = "LICENSE", default_value = "MIT")]
    pub license: String,

    /// Author
    #[arg(long, name = "AUTHOR", default_value = "Name <name@example.com>")]
    pub author: String,

    /// Version
    #[arg(long, name = "VERSION", default_value = "0.1.0")]
    pub version: String,

    /// Description
    #[arg(
        long,
        name = "DESCRIPTION",
        default_value = "Rust interface to Futhark library"
    )]
    pub description: String,

    /// Backend
    #[arg(long, name = "BACKEND", default_value = "c")]
    pub backend: Backend,
}

pub fn genfut(options: Options) {
    let name = options.name;
    let futhark_file = &options.file;
    let out_dir_str: String = format!("./{}", name);
    let out_dir = Path::new(&out_dir_str);

    // Create with create_dir_all, because we do not want to fail if
    // the directory already exists.
    if let Err(e) = create_dir_all(out_dir) {
        eprintln!("Error creating {:#?} ({})", out_dir, e);
        std::process::exit(1);
    }
    #[cfg(not(feature = "no_futhark"))]
    {
        let mut futhark_cmd = Command::new("futhark");

        let version_path = PathBuf::from(&out_dir).join("futhark-version.txt");
        let mut version_file =
            File::create(version_path).expect("could not create futhark-version.txt");
        futhark_cmd.arg("--version");
        let output = futhark_cmd.output().expect("failed: futhark --version");
        version_file
            .write_all(&output.stdout)
            .expect("failed to write Futhark version");
    }

    // Generate C code, Though only headerfiles are needed.
    // In general C files are generated when build at the user.
    gen_c(options.backend, futhark_file, out_dir);

    // copy futhark file
    if let Err(e) = std::fs::copy(futhark_file, PathBuf::from(out_dir).join("lib/a.fut")) {
        eprintln!("Error copying file: {}", e);
        std::process::exit(1);
    }

    #[cfg(not(all(feature = "opencl", target_os = "macos")))]
    {
        // Generate bindings
        let src_dir = PathBuf::from(out_dir).join("src");
        if let Err(e) = create_dir_all(&src_dir) {
            eprintln!("Error creating {:#?}, ({})", src_dir, e);
            std::process::exit(1);
        }

        generate_bindings(
            &PathBuf::from(out_dir).join("lib/a.h"),
            &PathBuf::from(out_dir).join("src"),
        );
    }

    let headers = std::fs::read_to_string(PathBuf::from(out_dir).join("lib/a.h"))
        .expect("Could not read headers");

    let re_array_types = Regex::new(r"struct (futhark_.+_\d+d)\s*;").expect("Regex failed!");
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
    let static_cargo = format!(
        include_str!("static/static_cargo.toml"),
        libname = name,
        description = &options.description,
        author = &options.author,
        version = &options.version,
        license = &options.license,
        backend = options.backend.to_feature(),
    );
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
}
