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

// Is there a case for having `send_sync` off?
// If not, we should probably add the wrapper from failure,
// https://github.com/rust-lang-nursery/failure/blob/master/src/sync_failure.rs
#[cfg(feature = "send_sync")]
pub(crate) type StrictError = dyn std::error::Error + Send + Sync + 'static;
#[cfg(not(feature = "send_sync"))]
pub(crate) type StrictError = dyn std::error::Error + 'static;
