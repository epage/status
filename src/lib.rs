mod macros;

mod alarm;
mod chain;
mod context;
mod internal;
mod kind;
mod term;

#[cfg(not(feature = "std"))]
compile_error!("no_std support is not implemented yet");

pub use crate::alarm::*;
pub use crate::chain::*;
pub use crate::context::*;
pub use crate::internal::*;
pub use crate::kind::*;
pub use crate::term::*;

pub(crate) type StdError = dyn std::error::Error + 'static;
