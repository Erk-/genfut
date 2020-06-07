extern crate genfut;
use genfut::{genfut, Opt};

fn main() {
    genfut(Opt {
        name: "matmul".to_string(),
        file: std::path::PathBuf::from("matmul.fut"),
        author: "Name <name@example.com>".to_string(),
        version: "0.1.0".to_string(),
        license: "YOLO".to_string(),
        description: "Futhark matrix multiplication example".to_string(),
    })
}
