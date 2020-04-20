#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]

mod arrays;
mod bindings;
mod context;
mod traits;

use std::ffi::CStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::os::raw::c_char;
use std::result::Result as StdResult;

pub use crate::arrays::*;
use crate::traits::*;
pub use context::FutharkContext;

#[derive(Debug)]
pub enum Error {
    FutharkError(FutharkError),
    SizeMismatch(usize, usize),
}

type Result<T> = StdResult<T, Error>;

impl From<FutharkError> for Error {
    fn from(err: FutharkError) -> Self {
        Error::FutharkError(err)
    }
}

#[derive(Debug)]
pub struct FutharkError {
    error: String,
}

impl FutharkError {
    pub(crate) fn new(ctx: *mut bindings::futhark_context) -> Self {
        unsafe { Self::_new(bindings::futhark_context_get_error(ctx)) }
    }

    pub(crate) fn _new(err: *mut ::std::os::raw::c_char) -> Self {
        unsafe {
            Self {
                error: CStr::from_ptr(err).to_string_lossy().into_owned(),
            }
        }
    }
}

impl Display for FutharkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.error)
    }
}

impl FutharkContext {
    pub fn matmul(&mut self, in0: Array_i32_2d, in1: Array_i32_2d) -> Result<(Array_i32_2d)> {
        let ctx = self.ptr();
        unsafe { _matmul(ctx, in0.as_raw_mut(), in1.as_raw_mut()) }
    }
}
unsafe fn _matmul(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_i32_2d,
    in1: *const bindings::futhark_i32_2d,
) -> Result<(Array_i32_2d)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_matmul(ctx, &mut raw_out0, in0, in1) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((Array_i32_2d::from_ptr(ctx, raw_out0)))
}
