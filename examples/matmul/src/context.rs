use crate::bindings;

#[derive(Clone, Copy)]
pub struct FutharkContext {
    context: *mut bindings::futhark_context,
    config: *mut bindings::futhark_context_config,
}

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

    pub(crate) fn ptr(&mut self) -> *mut bindings::futhark_context {
        self.context
    }
}

impl From<FutharkContext> for *mut bindings::futhark_context {
    fn from(mut ctx: FutharkContext) -> Self {
        ctx.ptr()
    }
}

