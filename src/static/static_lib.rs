#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]

mod bindings;
mod traits;
mod context;
mod arrays;

use crate::traits::*;
pub use crate::arrays::*;
pub use context::FutharkContext;
