use std::error::Error;
use std::fmt;

use crate::Alarm;
use crate::Kind;
use crate::Chain;
use crate::Context;

type StdError = dyn Error + 'static;

/// View of the error, exposing implementation details.
#[derive(Debug)]
pub struct InternalAlarm<K: Kind, C: Context>(Alarm<K, C>);

impl<K: Kind, C: Context> InternalAlarm<K, C> {
    pub(crate) fn new(err: Alarm<K, C>) -> Self {
        Self(err)
    }

    /// An iterator for the chain of sources.
    pub fn sources(&self) -> Chain {
        Chain::new(self.source())
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

impl<K: Kind, C: Context> fmt::Display for InternalAlarm<K, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl<K: Kind, C: Context> Error for InternalAlarm<K, C> {
    fn cause(&self) -> Option<&dyn Error> {
        (self.0).inner.source.any()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        (self.0).inner.source.any()
    }
}

