#[derive(Debug)]
pub struct {opaque_type}<'a> {{
    ptr: *const {futhark_type},
    ctx: &'a crate::context::FutharkContext,
}}

impl<'a> {opaque_type}<'a> {{
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

    pub(crate) unsafe fn free_opaque(&mut self)
    {{
        bindings::futhark_free_{base_type}(self.ctx.ptr(), self.as_raw_mut());
    }}
}}

impl Drop for {opaque_type}<'_> {{
    fn drop(&mut self) {{
        unsafe {{
            self.free_opaque();
        }}
    }}
}}
