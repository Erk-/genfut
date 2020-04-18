use crate::bindings;
use crate::traits::*;
use crate::{Error, Result};

pub(crate) trait FutharkType {
    type RustType: Default;
    const DIM: usize;
    
    unsafe fn shape<C>(ctx: C, ptr: *const Self) -> *mut i64
    where
        C: Into<*mut bindings::futhark_context>;
    unsafe fn values<C>(ctx: C, ptr: *mut Self, dst: *mut Self::RustType)
    where
        C: Into<*mut bindings::futhark_context>;
    unsafe fn free<C>(ctx: C, ptr: *mut Self)
    where
        C: Into<*mut bindings::futhark_context>;
}

use crate::bindings::*;

impl futhark_i32_2d {
   unsafe fn new<C>(ctx: C, arr: &[i32], dim: &[i64]) -> *const Self
   where C: Into<*mut bindings::futhark_context>
   {
     let ctx = ctx.into();
     bindings::futhark_new_i32_2d(
       ctx,
       arr.as_ptr() as *mut i32,
       dim[0],
dim[1],
)
     }
}

impl FutharkType for futhark_i32_2d {
   type RustType = i32;
   const DIM: usize = 2;

    unsafe fn shape<C>(ctx: C, ptr: *const bindings::futhark_i32_2d) -> *mut i64
    where C: Into<*mut bindings::futhark_context>
    {
        let ctx = ctx.into();
        bindings::futhark_shape_i32_2d(ctx, ptr as *mut bindings::futhark_i32_2d)
    }
    unsafe fn values<C>(ctx: C, ptr: *mut Self, dst: *mut Self::RustType)
    where C: Into<*mut bindings::futhark_context>
    {
        let ctx = ctx.into();
        bindings::futhark_values_i32_2d(ctx, ptr, dst);
    }
    unsafe fn free<C>(ctx: C, ptr: *mut Self)
    where C: Into<*mut bindings::futhark_context>
    {
        let ctx = ctx.into();
        bindings::futhark_free_i32_2d(ctx, ptr);
    }}
#[derive(Debug)]
pub struct Array_i32_2d {
    ptr: *const futhark_i32_2d,
    ctx: *mut bindings::futhark_context,
}


impl Array_i32_2d {
    pub(crate) unsafe fn as_raw(&self) -> *const futhark_i32_2d {
         self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut futhark_i32_2d {
         self.ptr as *mut futhark_i32_2d
    }
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const futhark_i32_2d) -> Self
        where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn shape<T>(ctx: T, ptr: *const futhark_i32_2d) -> Vec<i64>
    where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        let shape_ptr: *mut i64 = futhark_i32_2d::shape(ctx, ptr);
        let shape = std::slice::from_raw_parts(shape_ptr, 2);
        Vec::from(shape)
    }

    pub fn from_vec<T>(ctx: T, arr: &[i32], dim: &[i64]) -> Result<Self>
    where
        T: Into<*mut bindings::futhark_context>,
    {
        let expected = (dim.iter().fold(1, |acc, e| acc * e)) as usize;
        if arr.len() != expected {
            return Err(Error::SizeMismatch(arr.len(), expected));
        }

        let ctx = ctx.into();
        unsafe {
            let ptr = futhark_i32_2d::new(ctx, arr, dim);
            Ok(Array_i32_2d { ptr, ctx })
        }
    }
    
    pub fn to_vec(&self) -> (Vec<i32>, Vec<i64>)
    {
        let ctx = self.ctx;
        unsafe {
            futhark_context_sync(ctx);
            let shape = Self::shape(ctx, self.as_raw());
            let elems = shape.iter().fold(1, |acc, e| acc * e) as usize;
            let mut buffer: Vec<i32> =
                vec![i32::default(); elems];
            let cint = futhark_i32_2d::values(ctx, self.as_raw_mut(), buffer.as_mut_ptr());
            (buffer, shape.to_owned())
        }
    }

    pub(crate) unsafe fn free_array(&mut self)
    {
        futhark_i32_2d::free(self.ctx, self.as_raw_mut());
    }
}

impl Drop for Array_i32_2d {
    fn drop(&mut self) {
        unsafe {
            self.free_array();
        }
    }
}



