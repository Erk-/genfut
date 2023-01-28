use core::panic;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Backend {
    Cuda,
    ISPC,
    Multicore,
    OpenCL,
    C,
}

impl FromStr for Backend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.trim() {
            "cuda" => Ok(Backend::Cuda),
            "ispc" => Ok(Backend::ISPC),
            "multicore" => Ok(Backend::Multicore),
            "opencl" => Ok(Backend::OpenCL),
            "c" => Ok(Backend::C),
            _ => Err("Unknown backend, availible backends are: cuda, ispc, multicore (written in in c), none, opencl, c (sequential).".to_owned()),
        }
    }
}

impl Backend {
    pub(crate) fn to_feature(self) -> &'static str {
        match self {
            Backend::Cuda => "cuda",
            Backend::ISPC => "ispc",
            Backend::Multicore => "multicore",
            Backend::OpenCL => "opencl",
            Backend::C => "c",
        }
    }
}

pub(crate) fn gen_c(backend: Backend, in_file: &std::path::Path, out_dir: &std::path::Path) {
    let out_path = PathBuf::from(out_dir);
    let lib_dir = out_path.join("lib");
    if let Err(e) = create_dir_all(lib_dir.clone()) {
        eprintln!("Error creating {} ({})", lib_dir.display(), e);
        std::process::exit(1);
    }
    let output = Command::new("futhark")
        .arg(backend.to_feature())
        .arg("--library")
        .arg("-o")
        .arg(format!(
            "{}/lib/a",
            out_dir.to_str().expect("[gen_c] out_dir failed!")
        ))
        .arg(in_file)
        .output()
        .expect("[gen_c] failed to execute process");
    if !output.status.success() {
        println!(
            "Futhark stdout: {}",
            String::from_utf8(output.stdout).unwrap()
        );
        eprintln!(
            "Futhark stderr: {}",
            String::from_utf8(output.stderr).unwrap()
        );
        println!("Futhark status: {}", output.status);
        panic!("Futhark did not run successfully.")
    }
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
