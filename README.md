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

### Example on how to use a library generated with genfut

```rust
use matmul::{Array_i32_2d, Error, FutharkContext};

fn main() -> Result<(), Error> {
    let a = vec![1, 2, 3, 4];
    let b = vec![2, 3, 4, 1];

    let mut ctx = FutharkContext::new();

    let a_arr = Array_i32_2d::from_vec(ctx, &a, &vec![2, 2])?;
    let b_arr = Array_i32_2d::from_vec(ctx, &b, &vec![2, 2])?;

    let res_arr = ctx.matmul(a_arr, b_arr)?;

    let res = &res_arr.to_vec();

    for i in 0..4 {
        print!("{} ", res.0[i]);
        if i == 1 {
            print!("\n");
        }
    }
    print!("\n");
    println!("{:?}", res.0);
    Ok(())
}
```
