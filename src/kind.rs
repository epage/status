use std::fmt;

use crate::Status;

/// Programmatically describes which error occurred.
///
/// For a given `Kind`, it is expected that there is a single canonical schema for additional
/// data to be included.
///
/// # Example
///
/// ```rust
/// use status::Kind;
///
/// #[derive(Copy, Clone, Debug, derive_more::Display)]
/// enum ErrorKind {
///   #[display(fmt = "Failed to read file")]
///   Read,
///   #[display(fmt = "Failed to parse")]
///   Parse,
/// }
/// type Status = status::Status<ErrorKind>;
/// type Result<T, E = Status> = std::result::Result<T, E>;
///
/// fn read_file() -> Result<()> {
///     return ErrorKind::Read.into_err();
/// }
/// ```
pub trait Kind: Copy + Clone + fmt::Display + fmt::Debug + Send + Sync + 'static {
    /// Convenience for creating an error.
    fn into_status<C: crate::Context>(self) -> Status<Self, C> {
        Status::new(self)
    }

    /// Convenience for returning an error.
    fn into_err<T, C: crate::Context>(self) -> Result<T, Status<Self, C>> {
        Err(Status::new(self))
    }
}

impl<U> Kind for U where U: Copy + Clone + fmt::Display + fmt::Debug + Send + Sync + 'static {}
