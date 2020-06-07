# Genfut

This is a tool to generate a Rust library to interact with exported functions from a Futhark file.

## Usage

### As an executable binary
```shell
genfut <Rust lib name> <futhark_file.fut>
```

### Note that use of `bindings` module may not be generally portable. Use with caution.

### As a library

`build.rs`
```rust
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
```
