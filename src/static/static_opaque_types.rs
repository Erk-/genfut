#[derive(Debug)]
pub struct {opaque_type} {{
    ptr: *const {futhark_type},
    ctx: *mut bindings::futhark_context,
}}

impl {opaque_type} {{
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

    pub(crate) unsafe fn free_opaque(&mut self)
    {{
        if bindings::futhark_free_{base_type}(self.ctx, self.as_raw_mut()) != 0 {{
            panic!("Deallocation of object failed, this should not happen \
                    outside of compiler bugs and driver or hardware malfunction.");
        }}
    }}
}}

impl Drop for {opaque_type} {{
    fn drop(&mut self) {{
        unsafe {{
            self.free_opaque();
        }}
    }}
}}
