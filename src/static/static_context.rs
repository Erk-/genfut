use crate::bindings;

#[derive(Debug)]
pub struct FutharkContext {
    pub context: *mut bindings::futhark_context,
    pub config: *mut bindings::futhark_context_config,
}

// Safe to implement because Futhark has internal synchronization.
unsafe impl Sync for FutharkContext {}
unsafe impl Send for FutharkContext {}

impl FutharkContext {
    pub fn new() -> Self {
        unsafe {
            let ctx_config = bindings::futhark_context_config_new();
            let ctx = bindings::futhark_context_new(ctx_config);
            FutharkContext {
                context: ctx,
                config: ctx_config,
            }
        }
    }

    pub(crate) fn ptr(&self) -> *mut bindings::futhark_context {
        unsafe {
            std::mem::transmute(self.context)
        }
    }
}

impl From<FutharkContext> for *mut bindings::futhark_context {
    fn from(ctx: FutharkContext) -> Self {
        ctx.ptr()
    }
}

impl Drop for FutharkContext {
    fn drop(&mut self) {
        unsafe {
            bindings::futhark_context_free(self.context);
            bindings::futhark_context_config_free(self.config);
        }
    }
}
