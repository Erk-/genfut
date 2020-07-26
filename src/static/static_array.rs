use crate::bindings;
use crate::traits::*;
use crate::{Error, Result};
use crate::context::FutharkContext;

pub(crate) trait FutharkType {
    type RustType: Default;
    const DIM: usize;

    unsafe fn shape(ctx: &FutharkContext, ptr: *const Self) -> *const i64;
    unsafe fn values(ctx: &FutharkContext, ptr: *mut Self, dst: *mut Self::RustType);
    unsafe fn free(ctx: &FutharkContext, ptr: *mut Self);
}
