extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Sequential C support
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
    // FIXME: bindgen can't find OpenCL/cl.h on macos.
    #[cfg(all(feature = "opencl", not(target_os = "macos")))]
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

    #[cfg(not(all(feature = "opencl", target_os = "macos")))]
    {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
