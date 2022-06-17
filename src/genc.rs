use std::fs::create_dir_all;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Backend {
    Cuda,
    ISPC,
    MulticoreC,
    None,
    OpenCL,
    SequentialC,
}

impl FromStr for Backend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.trim() {
            "cuda" => Ok(Backend::Cuda),
            "ispc" => Ok(Backend::ISPC),
            "multicore_c" => Ok(Backend::MulticoreC),
            "none" => Ok(Backend::None),
            "opencl" => Ok(Backend::OpenCL),
            "sequential_c" => Ok(Backend::SequentialC),
            _ => Err("Unknown backend, availible backends are: cuda, ispc, multicore_c, none, opencl, sequential_c.".to_owned()),
        }
    }
}

impl Backend {
    pub(crate) fn to_feature(self) -> &'static str {
        match self {
            Backend::Cuda => "cuda",
            Backend::ISPC => "ispc",
            Backend::MulticoreC => "multicore_c",
            Backend::None => "none",
            Backend::OpenCL => "opencl",
            Backend::SequentialC => "sequential_c",
        }
    }
}

pub(crate) fn gen_c(backend: Backend, in_file: &std::path::Path, out_dir: &std::path::Path) {
    match backend {
        Backend::Cuda => cuda_gen_c(in_file, out_dir),
        Backend::ISPC => ispc_gen_c(in_file, out_dir),
        Backend::MulticoreC => multicore_gen_c(in_file, out_dir),
        Backend::None => { /* Intentionally left empty */ },
        Backend::OpenCL => opencl_gen_c(in_file, out_dir),
        Backend::SequentialC => seq_gen_c(in_file, out_dir),
    }
}

fn seq_gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {
    let out_path = PathBuf::from(out_dir);
    let lib_dir = out_path.join("lib");
    if let Err(e) = create_dir_all(lib_dir.clone()) {
        eprintln!("Error creating {} ({})", lib_dir.display(), e);
        std::process::exit(1);
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

fn multicore_gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {
    let out_path = PathBuf::from(out_dir);
    let lib_dir = out_path.join("lib");
    if let Err(e) = create_dir_all(lib_dir.clone()) {
        eprintln!("Error creating {} ({})", lib_dir.display(), e);
        std::process::exit(1);
    }
    let output = Command::new("futhark")
        .arg("multicore")
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

fn ispc_gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {
    let out_path = PathBuf::from(out_dir);
    let lib_dir = out_path.join("lib");
    if let Err(e) = create_dir_all(lib_dir.clone()) {
        eprintln!("Error creating {} ({})", lib_dir.display(), e);
        std::process::exit(1);
    }
    let output = Command::new("futhark")
        .arg("ispc")
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

fn cuda_gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {
    let out_path = PathBuf::from(out_dir);
    let lib_dir = out_path.join("lib");
    if let Err(e) = create_dir_all(lib_dir.clone()) {
        eprintln!("Error creating {} ({})", lib_dir.display(), e);
        std::process::exit(1);
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

fn opencl_gen_c(in_file: &std::path::Path, out_dir: &std::path::Path) {
    let out_path = PathBuf::from(out_dir);
    let lib_dir = out_path.join("lib");
    if let Err(e) = create_dir_all(lib_dir.clone()) {
        eprintln!("Error creating {} ({})", lib_dir.display(), e);
        std::process::exit(1);
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
