# Genfut

This is a tool to generate a rust libary to interact with exported functions from a futhark file.

## Usage

### As an executable binary
```shell
genfut <Rust lib name> <futhark_file.fut>
```

### As a library

`build.rs`
```rust
use genfut::genfut;

fn main() {
    genfut("<Rust lib name>", "futhark_file.fut")
}

```
