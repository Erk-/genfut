extern crate cc;

fn main() {
    // Sequential C support
    #[cfg(feature = "sequential_c")]
    cc::Build::new()
        .file("./lib/a.c")
        .flag("-fPIC")
        .flag("-std=c99")
        .shared_flag(true)
        .warnings(false)
        .compile("a");

    // CUDA support
    #[cfg(feature = "cuda")]
    cc::Build::new()
        .file("./lib/a.c")
        .cuda(true)
        .flag("-Xcompiler")
        .flag("-fPIC")
        .flag("-std=c99")
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
                .flag("-lOpenCL")
                .shared_flag(true)
                .compile("a");
            println!("cargo:rustc-link-lib=dylib=OpenCL");
        }
    }
}
