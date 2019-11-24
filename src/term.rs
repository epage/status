use std::error;
use std::fmt;

/// For use with `main`
///
/// # Example
///
/// ```rust,should_panic
/// fn main() -> Result<(), status::TerminatingStatus> {
///     Err(status::Status::new("Died"))?;
///     Ok(())
/// }
/// ```
pub struct TerminatingStatus<E: error::Error = crate::Status> {
    error: E,
}

impl<E: error::Error> From<E> for TerminatingStatus<E> {
    fn from(error: E) -> Self {
        Self { error }
    }
}

impl<E: error::Error> fmt::Debug for TerminatingStatus<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.error)?;
        for source in crate::Chain::new(self.error.source()) {
            writeln!(f)?;
            writeln!(f, "Caused by: {}", source)?;
        }
        Ok(())
    }
}
