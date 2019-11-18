use std::fmt;

use crate::Alarm;

/// Programmatically describes which error occurred.
///
/// ```rust
/// use alarm::AlarmKind;
///
/// #[derive(Copy, Clone, Debug, derive_more::Display)]
/// enum ErrorKind {
///   #[display(fmt = "Failed to read file")]
///   Read,
///   #[display(fmt = "Failed to parse")]
///   Parse,
/// }
/// impl AlarmKind for ErrorKind {
///     type Data = alarm::NoData;
/// }
/// type Alarm = alarm::Alarm<ErrorKind>;
/// type Result<T, E = Alarm> = std::result::Result<T, E>;
///
/// pub fn read_file() -> Result<()> {
///     return ErrorKind::Read.into_err();
/// }
/// ```
pub trait AlarmKind: Copy + Clone + fmt::Display + fmt::Debug + Send + Sync + 'static {
    type Data: crate::Data;

    /// Convenience for creating an error.
    fn into_alarm(self) -> Alarm<Self> {
        Alarm::new(self)
    }

    /// Convenience for returning an error.
    fn into_err<T>(self) -> Result<T, Alarm<Self>> {
        Err(Alarm::new(self))
    }
}
