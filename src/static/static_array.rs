use crate::bindings;
use crate::traits::*;
use crate::{Error, Result};
use std::os::raw::c_int;

pub(crate) trait FutharkType {
    type RustType: Default;
    const DIM: usize;

    unsafe fn shape<C>(ctx: C, ptr: *const Self) -> *const i64
    where
        C: Into<*mut bindings::futhark_context>;
    unsafe fn values<C>(ctx: C, ptr: *mut Self, dst: *mut Self::RustType) -> Result<()>
    where
        C: Into<*mut bindings::futhark_context>;
    unsafe fn free<C>(ctx: C, ptr: *mut Self) -> c_int
    where
        C: Into<*mut bindings::futhark_context>;
}
