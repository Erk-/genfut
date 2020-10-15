//! Example of how to use the built matmul library

use matmul::{Array_i32_2d, Error, FutharkContext};

fn main() -> Result<(), Error> {
    let a = vec![1, 2, 3, 4];
    let b = vec![2, 3, 4, 1];

    let mut ctx = FutharkContext::new()?;

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
