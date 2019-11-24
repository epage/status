use std::fmt;

use crate::Status;

/// Trait alias for types that programmatically specify the status.
///
/// For prototyping, see [`Unkind`].
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

/// Adhoc [`Kind`].
///
/// Unlike most [`Kind`]s, this is meant to be opaque and not programmatically specify the status.
/// It is only good for displaying a `str` to the user when prototyping before one transitions to more formal [`Kind`]s.
///
/// Note: This is the default [`Kind`] for [`Status`].
///
/// When transitioning to a more useful [`Kind`], it could be helpful to have an `enum` variant
/// with an `Unkind`:
/// ```
/// #[derive(Copy, Clone, Debug, derive_more::Display)]
/// enum ErrorKind {
///   #[display(fmt = "Failed to read file")]
///   Read,
///   #[display(fmt = "Failed to parse")]
///   Parse,
///   #[display(fmt = "{}", "_0")]
///   Other(status::Unkind),
/// }
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Unkind {
    inner: &'static str,
}

impl From<&'static str> for Unkind {
    fn from(s: &'static str) -> Self {
        Self { inner: s }
    }
}

impl fmt::Display for Unkind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use static_assertions::*;

    #[test]
    fn unkind() {
        assert_impl_all!(Unkind: Kind);
    }
}
