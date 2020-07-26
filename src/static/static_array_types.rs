#[derive(Debug)]
pub struct {array_type}<'a> {{
    ptr: *const {futhark_type},
    ctx: &'a crate::context::FutharkContext,
}}


impl<'a> {array_type}<'a> {{
    pub(crate) unsafe fn as_raw(&self) -> *const {futhark_type} {{
         self.ptr
    }}

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut {futhark_type} {{
         self.ptr as *mut {futhark_type}
    }}
    pub(crate) unsafe fn from_ptr(ctx: &'a crate::context::FutharkContext, ptr: *const {futhark_type}) -> Self
    {{
        Self {{ ptr, ctx }}
    }}

    pub(crate) unsafe fn shape(ctx: &crate::context::FutharkContext, ptr: *const {futhark_type}) -> Vec<i64>
    {{
        let shape_ptr: *const i64 = {futhark_type}::shape(ctx, ptr);
        let shape = std::slice::from_raw_parts(shape_ptr, {dim});
        Vec::from(shape)
    }}

    pub fn from_vec(ctx: &'a crate::context::FutharkContext, arr: &[{inner_type}], dim: &[i64]) -> Result<Self>
    {{
        let expected = (dim.iter().fold(1, |acc, e| acc * e)) as usize;
        if arr.len() != expected {{
            return Err(Error::SizeMismatch(arr.len(), expected));
        }}

        unsafe {{
            let ptr = {futhark_type}::new(ctx, arr, dim);
            Ok({array_type} {{ ptr, ctx }})
        }}
    }}
    
    pub fn to_vec(&self) -> (Vec<{inner_type}>, Vec<i64>)
    {{
        let ctx = self.ctx;
        unsafe {{
            futhark_context_sync(ctx.ptr());
            let shape = Self::shape(ctx, self.as_raw());
            let elems = shape.iter().fold(1, |acc, e| acc * e) as usize;
            let mut buffer: Vec<{inner_type}> =
                vec![{inner_type}::default(); elems];
            let cint = {futhark_type}::values(ctx, self.as_raw_mut(), buffer.as_mut_ptr());
            (buffer, shape.to_owned())
        }}
    }}

    pub(crate) unsafe fn free_array(&mut self)
    {{
        {futhark_type}::free(self.ctx, self.as_raw_mut());
    }}
}}

impl Drop for {array_type}<'_> {{
    fn drop(&mut self) {{
        unsafe {{
            self.free_array();
        }}
    }}
}}
