# Genfut

This is a tool to generate a Rust library to interact with exported functions from a Futhark file.

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
