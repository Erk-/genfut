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
use genfut::genfut;

fn main() {
    genfut("<Rust lib name>", "futhark_file.fut")
}

```
