extern crate cc;

fn main() {
    // Sequential C support
    #[cfg(feature = "c")]
    cc::Build::new()
        .file("./lib/a.c")
        .flag("-fPIC")
        .flag("-std=c99")
        .flag("-O3")
        .shared_flag(true)
        .warnings(false)
        .compile("a");

    // Multicore C support
    #[cfg(feature = "multicore")]
    cc::Build::new()
        .file("./lib/a.c")
        .flag("-fPIC")
        .flag("-pthread")
        .flag("-lm")
        .flag("-O3")
        .flag("-std=c99")
        .shared_flag(true)
        .warnings(false)
        .compile("a");

    // Multicore ISPC support
    #[cfg(feature = "ispc")]
    {
        let mut ispc = std::process::Command::new("ispc");
        ispc.arg("./lib/a.kernels.ispc")
            .arg("-o")
            .arg("./lib/a.kernels.o")
            .arg("--pic")
            .arg("-O3")
            .arg("--addressing=64")
            .arg("--target=host");
        ispc.output().expect("Failed to invoke ispc.");

        cc::Build::new()
            .file("./lib/a.c")
            .object("./lib/a.kernels.o")
            .flag("-fPIC")
            .flag("-O3")
            .flag("-pthread")
            .flag("-lm")
            .flag("-std=c99")
            .shared_flag(true)
            .warnings(false)
            .compile("a");
    }

    // CUDA support
    #[cfg(feature = "cuda")]
    cc::Build::new()
        .file("./lib/a.c")
        .cuda(true)
        .flag("-Xcompiler")
        .flag("-fPIC")
        .flag("-std=c++03")
        .flag("-O3")
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
    {
        #[cfg(not(target_os = "macos"))]
        {
            cc::Build::new()
                .file("./lib/a.c")
                .flag("-fPIC")
                .flag("-std=c99")
                .flag("-O3")
                .shared_flag(true)
                .compile("a");
            println!("cargo:rustc-link-lib=dylib=OpenCL");
        }
        #[cfg(target_os = "macos")]
        {
            cc::Build::new()
                .file("./lib/a.c")
                .flag("-fPIC")
                .flag("-std=c99")
                .flag("-O3")
                .shared_flag(true)
                .compile("a");
            println!("cargo:rustc-link-lib=framework=OpenCL");
        }
    }
}
