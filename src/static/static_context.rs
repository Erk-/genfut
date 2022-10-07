use crate::bindings;
use crate::{FutharkError, Result};

#[derive(Clone, Copy)]
pub struct FutharkContext {
    pub context: *mut bindings::futhark_context,
    pub config: *mut bindings::futhark_context_config,
}

// impl Drop for FutharkContext {

// }

// Safe to implement because Futhark has internal synchronization.
unsafe impl Sync for FutharkContext {}
unsafe impl Send for FutharkContext {}

// I recommend adding an implementation of the Drop trait for
// FutharkContext, which should call futhark_context_free() and
// futhark_context_config_free() (in that order)

impl FutharkContext {
    pub fn new() -> Result<Self> {
        unsafe {
            let ctx_config = bindings::futhark_context_config_new();
            let ctx = bindings::futhark_context_new(ctx_config);

            let err = bindings::futhark_context_get_error(ctx);

            if err.is_null() {
                Ok(FutharkContext {
                    context: ctx,
                    config: ctx_config,
                })
            } else {
                Err(FutharkError::_new(err).into())
            }
        }
    }

    pub fn free(&mut self) {
        unsafe {
            bindings::futhark_context_free(self.context);
            bindings::futhark_context_config_free(self.config);
        }
    }

    pub(crate) fn ptr(&mut self) -> *mut bindings::futhark_context {
        self.context
    }
}

impl From<FutharkContext> for *mut bindings::futhark_context {
    fn from(mut ctx: FutharkContext) -> Self {
        ctx.ptr()
    }
}
