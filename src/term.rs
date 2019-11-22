use std::error;
use std::fmt;

/// For use with `main`
///
/// # Example
///
/// ```rust
/// fn main() -> Result<(), alarm::TerminatingAlarm> {
///     Err(alarm::Alarm::new("Died"))?;
///     Ok(())
/// }
/// ```
pub struct TerminatingAlarm<E: error::Error = crate::Alarm> {
    error: E,
}

impl<E: error::Error> From<E> for TerminatingAlarm<E> {
    fn from(error: E) -> Self {
        Self {
            error
        }
    }
}

impl<E: error::Error> fmt::Debug for TerminatingAlarm<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.error)?;
        for source in crate::Chain::new(self.error.source()) {
            writeln!(f)?;
            writeln!(f, "Caused by: {}", source)?;
        }
        Ok(())
    }
}
