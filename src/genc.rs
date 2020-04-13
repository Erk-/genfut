use std::fs::create_dir;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

#[cfg(feature = "no_futhark")]
pub(crate) fn gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {}

#[cfg(feature = "sequential_c")]
pub(crate) fn gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {
    let out_path = PathBuf::from(out_dir);
    if let Err(e) = create_dir(out_path.join("lib")) {
        println!("Error creating dir ({})", e);
    }
    let output = Command::new("futhark")
        .arg("c")
        .arg("--library")
        .arg("-o")
        .arg(format!(
            "{}/lib/a",
            out_dir.to_str().expect("[gen_c] out_dir failed!")
        ))
        .arg(in_file)
        .output()
        .expect("[gen_c] failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

#[cfg(feature = "cuda")]
pub(crate) fn gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {
    let out_path = PathBuf::from(out_dir);
    if let Err(e) = create_dir(out_path.join("lib")) {
        println!("Error creating dir ({})", e);
    }
    let output = Command::new("futhark")
        .arg("cuda")
        .arg("--library")
        .arg("-o")
        .arg(format!(
            "{}/lib/a",
            out_dir.to_str().expect("[gen_c] out_dir failed!")
        ))
        .arg(in_file)
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

#[cfg(feature = "opencl")]
pub(crate) fn gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {
    let out_path = PathBuf::from(out_dir);
    if let Err(e) = create_dir(out_path.join("lib")) {
        println!("Error creating dir ({})", e);
    }
    let output = Command::new("futhark")
        .arg("opencl")
        .arg("--library")
        .arg("-o")
        .arg(format!(
            "{}/lib/a",
            out_dir.to_str().expect("[gen_c] out_dir failed!")
        ))
        .arg(in_file)
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

pub(crate) fn generate_bindings(header: &std::path::Path, out: &std::path::Path) {
    let bindings = bindgen::Builder::default()
        .header(
            header
                .to_str()
                .expect("[generate_bindings] Error with header!"),
        )
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(out);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
