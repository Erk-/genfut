use crate::bindings;
use crate::traits::*;
use crate::{Error, Result};

pub(crate) trait FutharkType {
    type RustType: Default;
    const DIM: usize;

    unsafe fn shape(ctx: &crate::context::FutharkContext, ptr: *const Self) -> *const i64;
    unsafe fn values(ctx: &crate::context::FutharkContext, ptr: *mut Self, dst: *mut Self::RustType);
    unsafe fn free(ctx: &crate::context::FutharkContext, ptr: *mut Self);
}
