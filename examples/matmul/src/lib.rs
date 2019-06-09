#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]

mod bindings;
mod traits;
mod context;
mod arrays;

use crate::traits::*;
pub use crate::arrays::*;
pub use context::FutharkContext;

impl FutharkContext {
pub fn matmul(&mut self, in0: Array_i32_2d, in1: Array_i32_2d, ) -> (Array_i32_2d)
{
let ctx = self.ptr();
unsafe{
_matmul(ctx, in0.as_raw_mut(), in1.as_raw_mut(), )
}}

}
unsafe fn _matmul(ctx: *mut bindings::futhark_context, in0: *const bindings::futhark_i32_2d, in1: *const bindings::futhark_i32_2d, ) -> (Array_i32_2d) {
let mut raw_out0 = std::ptr::null_mut();

bindings::futhark_entry_matmul(ctx, &mut raw_out0, in0, in1, );
(Array_i32_2d::from_ptr(ctx, raw_out0)
)
}


