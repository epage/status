//! [`Status`]: An `Error` container for Rust.
//!
//! An `Error` container lowers the overhead for reporting the status via `Result<_, E>`.
//!
//! Unlike the error-wrapping pattern found in `cargo` and generalized in `anyhow`, the pattern
//! implemented in [`Status`] comes from projects which try to address the
//! following requirements:
//! - Programmatically respond to both the [`Kind`] of status and the metadata, or [`Context`], of
//!   the status.
//! - Dealing with error-sites not knowing enough to describe the error but allowing the
//!   [`Context`] to be built gradually when unwinding where there is relevant information to
//!   add.
//! - Localizing the rendered message.
//! - Allowing an application to make some phrasing native to its UX.
//! - Preserving all of this while passing through FFI, IPC, and RPC (TODO [#1](https://github.com/epage/status/issues/1) [#2](https://github.com/epage/status/issues/2)).
//!
//! These requirements are addressed by trading off the usability of per-site custom messages with
//! messages built up from common building blocks.  The [`Kind`] serves as a static description of the
//! error that comes from a general, fixed collection.  Describing the exact problem and tailored
//! remediation is the responsibility of the [`Context`] which maps general, fixed keys with
//! runtime-generated data.
//!
//! [`Status`] grows with your application:
//!
//! # Prototyping
//!
//! When prototyping, you tend to focus on your proof of concept and not worry about handling all
//! corner cases or providing a clean API.  `status` helps you with this by providing an ad-hoc
//! [`Kind`], [`Unkind`], and an [`AdhocContext`].
//!
//! ```rust
//! # use std::path::Path;
//! type Status = status::Status;
//! type Result<T, E = Status> = std::result::Result<T, E>;
//!
//! fn read_file(path: &Path) -> Result<String> {
//!     std::fs::read_to_string(path)
//!         .map_err(|e| {
//!             Status::new("Failed to read file")
//!                 .with_internal(e)
//!                 .context_with(|c| c.insert("Expected value", 5))
//!         })
//! }
//!
//! fn main() -> Result<(), status::TerminatingStatus> {
//!     let content = read_file(Path::new("Cargo.toml"))?;
//!     println!("{}", content);
//!     Ok(())
//! }
//! ```
//!
//! # Maturing the code
//!
//! After prototyping, you probably want to start handling all the corner cases and have a more
//! strictly defined API.  You can do this by using an `enum` for your [`Kind`].  As you play
//! whack-a-mole in cleaning up error handling, you might want to still allow some ad-hoc
//! [`Kind`]s.
//!
//! ```rust
//! # use std::path::Path;
//! #[derive(Copy, Clone, Debug, derive_more::Display, derive_more::From)]
//! enum ErrorKind {
//!   #[display(fmt = "Failed to parse")]
//!   Parse,
//!   #[display(fmt = "{}", "_0")]
//!   #[doc(hidden)]
//!   __Other(&'static str),
//! }
//! type Status = status::Status<ErrorKind>;
//! type Result<T, E = Status> = std::result::Result<T, E>;
//!
//! fn read_file(path: &Path) -> Result<String, Status> {
//!     std::fs::read_to_string(path)
//!         .map_err(|e| {
//!             Status::new("Failed to read file").with_internal(e)
//!         })
//! }
//!
//! # fn main() -> Result<(), status::TerminatingStatus<Status>> {
//! #     let content = read_file(Path::new("Cargo.toml"))?;
//! #     println!("{}", content);
//! #     Ok(())
//! # }
//! ```
//!
//! And once you are done, you might choose to remove `ErrorKind::__Other` completely:
//!
//! ```rust
//! # use std::path::Path;
//! #[derive(Copy, Clone, Debug, derive_more::Display)]
//! enum ErrorKind {
//!   #[display(fmt = "Failed to read file")]
//!   Read,
//!   #[display(fmt = "Failed to parse")]
//!   Parse,
//! }
//! # type Status = status::Status<ErrorKind>;
//! # type Result<T, E = Status> = std::result::Result<T, E>;
//! #
//! fn read_file(path: &Path) -> Result<String, Status> {
//!     std::fs::read_to_string(path)
//!         .map_err(|e| {
//!             Status::new(ErrorKind::Read).with_internal(e)
//!         })
//! }
//! #
//! # fn main() -> Result<(), status::TerminatingStatus<Status>> {
//! #     let content = read_file(Path::new("Cargo.toml"))?;
//! #     println!("{}", content);
//! #     Ok(())
//! # }
//! ```
//!
//! The same progressions happens with [`Context`], from [`AdhocContext`] to hand-written
//! [`Context`].
//!
//! # FAQ
//!
//! ## Why `Status`?
//!
//! For an "error" crate that wanted to focus on the programmatic use-case, the typical synonyms for
//! "error" were too strong  because one man's error is another man's expected case.  For example,
//! you might have a case where you need to silence some "errors" and move on.
//!
//! ## When should my [`Kind`] be an `enum` or an error code?
//!
//! In the above examples, `enum`s were used when maturing the code but error codes still have a
//! place.
//!
//! `enum`s:
//! - Strictly typed
//!
//! Error codes:
//! - FFI, IPC, and RPC between binaries with different iterations of the error codes.
//! - Localizaton: An approach to localization is lookup tables. Keeping them separate from the
//!   application allows updating them without recompiling.
//! - Interoperating with an ecosystem standardized on an error code system (like `HRESULT`).
//!
//! When using error codes, be sure to wrap them in a newtype to avoid mixing meanings.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

mod macros;

mod chain;
mod context;
mod ext;
mod internal;
mod kind;
mod status;
mod term;

#[cfg(not(feature = "std"))]
compile_error!("no_std support is not implemented yet");

pub use crate::chain::*;
pub use crate::context::*;
pub use crate::ext::*;
pub use crate::internal::*;
pub use crate::kind::*;
pub use crate::status::*;
pub use crate::term::*;

pub(crate) type StdError = dyn std::error::Error + 'static;

// Is there a case for having `send_sync` off?
// If not, we should probably add the wrapper from failure,
// https://github.com/rust-lang-nursery/failure/blob/master/src/sync_failure.rs
#[cfg(feature = "send_sync")]
pub(crate) type StrictError = dyn std::error::Error + Send + Sync + 'static;
#[cfg(not(feature = "send_sync"))]
pub(crate) type StrictError = dyn std::error::Error + 'static;
