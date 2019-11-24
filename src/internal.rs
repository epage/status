use std::error;
use std::fmt;

use crate::Chain;
use crate::Context;
use crate::Kind;
use crate::Status;
use crate::StdError;

/// View of the error, exposing implementation details.
#[derive(Debug)]
pub struct InternalStatus<K: Kind, C: Context>(Status<K, C>);

impl<K: Kind, C: Context> InternalStatus<K, C> {
    pub(crate) fn new(err: Status<K, C>) -> Self {
        Self(err)
    }

    /// An iterator for the chain of sources.
    pub fn sources(&self) -> Chain {
        Chain::new(error::Error::source(self))
    }

    /// The lowest level cause of this error &mdash; this error's cause's
    /// cause's cause etc.
    ///
    /// The root cause is the last error in the iterator produced by
    /// [`chain()`][Error::chain].
    pub fn root_source(&self) -> Option<&StdError> {
        self.sources().last()
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
