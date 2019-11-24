use std::error;
use std::fmt;

use crate::Chain;
use crate::Context;
use crate::Kind;
use crate::Status;

/// View of [`Status`], exposing implementation details.
///
/// `Error::source` and [`InternalStatus::sources`] are for debug / display purposes only and
/// relying on them programmatically is likely to break across minor releases.
///
/// # Example
///
/// ```
/// fn display_status(status: status::Status) {
///     if std::env::var("DEBUG").as_ref().map(|s| s.as_ref()).unwrap_or("0") == "1" {
///         let status = status.into_internal();
///         println!("{}", status);
///     } else {
///         println!("{}", status);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct InternalStatus<K: Kind, C: Context>(Status<K, C>);

impl<K: Kind, C: Context> InternalStatus<K, C> {
    pub(crate) fn new(err: Status<K, C>) -> Self {
        Self(err)
    }

    /// An iterator for the chain of sources, private or public.
    pub fn sources(&self) -> Chain {
        Chain::new(error::Error::source(self))
    }
}

impl<K: Kind, C: Context> fmt::Display for InternalStatus<K, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl<K: Kind, C: Context> error::Error for InternalStatus<K, C> {
    fn cause(&self) -> Option<&dyn error::Error> {
        (self.0).inner.source.any()
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        (self.0).inner.source.any()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NoContext;
    use crate::Unkind;

    use static_assertions::*;

    #[test]
    fn internal() {
        assert_impl_all!(InternalStatus<Unkind, NoContext>: fmt::Debug, fmt::Display, error::Error);
        #[cfg(feature = "send_sync")]
        assert_impl_all!(InternalStatus<Unkind, NoContext>: Send, Sync);
    }
}
