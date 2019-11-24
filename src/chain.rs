type StdError = dyn std::error::Error + 'static;

/// Iterator of a chain of source errors.
///
/// [`Status::sources`] will only return errors that are part of the API / user-visible. To access
/// debug / internal information, see [`InternalStatus::sources`].
///
/// # Example
///
/// ```
/// use status::Status;
/// use std::io;
///
/// pub fn underlying_io_error_kind(error: &Status) -> Option<io::ErrorKind> {
///     for cause in error.sources() {
///         if let Some(io_error) = cause.downcast_ref::<io::Error>() {
///             return Some(io_error.kind());
///         }
///     }
///     None
/// }
/// ```
#[derive(Debug)]
pub struct Chain<'a> {
    next: Option<&'a StdError>,
}

impl<'a> Chain<'a> {
    pub(crate) fn new(next: Option<&'a StdError>) -> Self {
        Self { next }
    }
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a StdError;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take()?;
        self.next = next.source();
        Some(next)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use static_assertions::*;

    #[test]
    fn chain() {
        assert_impl_all!(Chain: std::fmt::Debug);
    }
}
