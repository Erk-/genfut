#[derive(Debug)]
pub struct {array_type} {{
    ptr: *const {futhark_type},
    ctx: *mut bindings::futhark_context,
}}

impl {array_type} {{
    pub(crate) unsafe fn as_raw(&self) -> *const {futhark_type} {{
         self.ptr
    }}

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut {futhark_type} {{
         self.ptr as *mut {futhark_type}
    }}
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const {futhark_type}) -> Self
        where
        T: Into<*mut bindings::futhark_context>,
    {{
        let ctx = ctx.into();
        Self {{ ptr, ctx }}
    }}

    pub(crate) unsafe fn shape<T>(ctx: T, ptr: *const {futhark_type}) -> Vec<i64>
    where
        T: Into<*mut bindings::futhark_context>,
    {{
        let ctx = ctx.into();
        let shape_ptr: *const i64 = {futhark_type}::shape(ctx, ptr);
        let shape = std::slice::from_raw_parts(shape_ptr, {dim});
        Vec::from(shape)
    }}

    pub fn from_vec<T>(ctx: T, arr: &[{inner_type}], dim: &[i64]) -> Result<Self>
    where
        T: Into<*mut bindings::futhark_context>,
    {{
        let expected = (dim.iter().fold(1, |acc, e| acc * e)) as usize;
        if arr.len() != expected {{
            return Err(Error::SizeMismatch(arr.len(), expected));
        }}

        let ctx = ctx.into();
        unsafe {{
            let ptr = {futhark_type}::new(ctx, arr, dim);
            Ok({array_type} {{ ptr, ctx }})
        }}
    }}
    
    pub fn to_vec(&self) -> Result<(Vec<{inner_type}>, Vec<i64>)>
    {{
        let ctx = self.ctx;
        unsafe {{
            futhark_context_sync(ctx);
            let shape = Self::shape(ctx, self.as_raw());      
            let elems = shape.iter().fold(1, |acc, e| acc * e) as usize;
            let mut buffer: Vec<{inner_type}> =
                vec![{inner_type}::default(); elems];
            {futhark_type}::values(ctx, self.as_raw_mut(), buffer.as_mut_ptr())?;
            Ok((buffer, shape))
        }}
    }}

    pub(crate) unsafe fn free_array(&mut self)
    {{
        if {futhark_type}::free(self.ctx, self.as_raw_mut()) != 0 {{
            panic!("Deallocation of object failed, this should not happen \
                    outside of compiler bugs and driver or hardware malfunction.");
        }}
    }}
}}

impl Drop for {array_type} {{
    fn drop(&mut self) {{
        unsafe {{
            self.free_array();
        }}
    }}
}}
