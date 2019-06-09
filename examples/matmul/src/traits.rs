use crate::bindings::futhark_context;

pub(crate) trait FromCtx<T>: Sized {
    /// Performs the conversion with the futhark context.
    fn from_ctx(_: T, ctx: *mut futhark_context) -> Self;
}

pub(crate) trait IntoCtx<T>: Sized {
    /// Performs the conversion with the futhark context.
    fn into_ctx(self, ctx: *mut futhark_context) -> T;
}

impl<T, U> IntoCtx<U> for T
where
    U: FromCtx<T>,
{
    fn into_ctx(self, ctx: *mut futhark_context) -> U {
        U::from_ctx(self, ctx)
    }
}

