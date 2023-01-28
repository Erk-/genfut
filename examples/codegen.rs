//! Binary to generate the matmul library

extern crate genfut;
use genfut::{genfut, Backend, Options};

fn main() {
    genfut(Options {
        name: "matmul".to_string(),
        file: std::path::PathBuf::from("./matmul.fut"),
        author: "Name <name@example.com>".to_string(),
        version: "0.1.0".to_string(),
        license: "MIT".to_string(),
        description: "Futhark matrix multiplication example".to_string(),
        backend: Backend::C,
    })
}
