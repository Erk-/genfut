extern crate bindgen;
extern crate cc;

use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Sequential C support
    #[cfg(feature = "sequential_c")]
    let _ = Command::new("futhark")
        .arg("c")
        .arg("--library")
        .arg("./lib/a.fut")
        .output()
        .expect("failed to execute process");
    #[cfg(feature = "sequential_c")]
    let bindings = bindgen::Builder::default()
        .header("./lib/a.h")
        .generate()
        .expect("Unable to generate bindings");
    #[cfg(feature = "sequential_c")]
    cc::Build::new()
        .file("./lib/a.c")
        .flag("-fPIC")
        .shared_flag(true)
        .warnings(false)
        .compile("a");

    // CUDA support
    #[cfg(feature = "cuda")]
    let _ = Command::new("futhark")
        .arg("cuda")
        .arg("--library")
        .arg("./lib/a.fut")
        .output()
        .expect("failed to execute process");
    #[cfg(feature = "cuda")]
    let bindings = bindgen::Builder::default()
        .header("./lib/a.h")
        .clang_arg("-I/opt/cuda/include")
        .generate()
        .expect("Unable to generate bindings");
    #[cfg(feature = "cuda")]
    cc::Build::new()
        .file("./lib/a.c")
        .cuda(true)
        .flag("-Xcompiler")
        .flag("-fPIC")
        .flag("-w")
        .shared_flag(true)
        .compile("a");
    #[cfg(feature = "cuda")]
    {
        println!("cargo:rustc-link-search=native=/opt/cuda/include");
        println!("cargo:rustc-link-search=native=/opt/cuda/lib64");
        println!("cargo:rustc-link-lib=dylib=cuda");
        println!("cargo:rustc-link-lib=dylib=nvrtc");
    }

    // OpenCL support
    #[cfg(feature = "opencl")]
    let _ = Command::new("futhark")
        .arg("opencl")
        .arg("--library")
        .arg("./lib/a.fut")
        .output()
        .expect("failed to execute process");
    #[cfg(feature = "opencl")]
    let bindings = bindgen::Builder::default()
        .header("./lib/a.h")
        .generate()
        .expect("Unable to generate bindings");
    #[cfg(feature = "opencl")]
    {
        #[cfg(not(target_os = "macos"))]
        {
            cc::Build::new()
                .file("./lib/a.c")
                .flag("-fPIC")
                .flag("-lOpenCL")
                .shared_flag(true)
                .compile("a");
            println!("cargo:rustc-link-lib=dylib=OpenCL");
        }
        #[cfg(target_os = "macos")]
        {
            cc::Build::new()
                .file("./lib/a.c")
                .flag("-fPIC")
                .flag("-framework")
                .flag("OpenCL")
                .shared_flag(true)
                .compile("a");
        }
    }

    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
