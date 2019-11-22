use std::error;
use std::fmt;

use crate::Kind;
use crate::Chain;
use crate::Context;
use crate::InternalAlarm;
use crate::NoContext;
use crate::StdError;

/// Error container.
///
/// Goals:
/// - Easy crate inter-op while maintaining programmatic processing.
/// - User-friendly without losing helpful debug information.
///
/// Note: this is optimized for the happy-path.  When failing frequently inside of an inner loop,
/// consider implementing `Error` on your `Kind`.
///
/// # Example
///
/// ```rust
/// use alarm::Kind;
///
/// #[derive(Copy, Clone, Debug, derive_more::Display)]
/// enum ErrorKind {
///   #[display(fmt = "Failed to read file")]
///   Read,
///   #[display(fmt = "Failed to parse")]
///   Parse,
/// }
/// type Alarm = alarm::Alarm<ErrorKind>;
/// type Result<T, E = Alarm> = std::result::Result<T, E>;
///
/// fn read_file() -> Result<()> {
///     return ErrorKind::Read.into_err();
/// }
/// ```
#[derive(Debug)]
pub struct Alarm<K: Kind = &'static str, C: Context = NoContext> {
    pub(crate) inner: Box<AlarmDetails<K, C>>
}

#[derive(Debug)]
pub(crate) struct AlarmDetails<K: Kind, C: Context> {
    pub(crate) kind: K,
    pub(crate) source: Source,
    pub(crate) data: C,
}

impl<K: Kind, C: Context> Alarm<K, C> {
    /// Create a new error object from the error kind.
    pub fn new(kind: K) -> Self {
        Self { inner: Box::new(AlarmDetails {
            kind,
            source: Source::Empty,
            data: Default::default(),
        })}
    }

    /// Add a public error.
    pub fn with_source<E>(mut self, error: E) -> Self
        where E: error::Error + 'static
    {
        self.inner.source = Source::Public(Box::new(error));
        self
    }

    /// Add an internal error.
    pub fn with_internal<E>(mut self, error: E) -> Self
        where E: error::Error + 'static
    {
        self.inner.source = Source::Private(Box::new(error));
        self
    }

    /// Programmatic identifier for which error occurred.
    pub fn kind(&self) -> K {
        self.inner.kind
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

    /// View of the error, exposing implementation details.
    pub fn into_internal(self) -> InternalAlarm<K, C> {
        InternalAlarm::new(self)
    }

    /// Convenience for returning an error.
    pub fn into_err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl<K: Kind, C: Context> fmt::Display for Alarm<K, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.inner.kind)?;
        if !self.inner.data.is_empty() {
            writeln!(f)?;
            writeln!(f, "{}", self.inner.data)?;
        }
        Ok(())
    }
}

impl<K: Kind, C: Context> std::ops::Deref for Alarm<K, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.inner.data
    }
}

impl<K: Kind, C: Context> std::ops::DerefMut for Alarm<K, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner.data
    }
}

impl<K: Kind, C: Context> error::Error for Alarm<K, C> {
    fn cause(&self) -> Option<&dyn error::Error> {
        self.inner.source.public()
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.inner.source.public()
    }
}

// impl From<Kind> is waiting on specialization

// impl From<Error> is waiting on specialization

#[derive(Debug)]
pub(crate) enum Source {
    Public(Box<StdError>),
    Private(Box<StdError>),
    Empty,
}

impl Source {
    pub(crate) fn public(&self) -> Option<&StdError> {
        match self {
            Self::Public(e) => Some(e.as_ref()),
            _ => None,
        }
    }

    pub(crate) fn any(&self) -> Option<&StdError> {
        match self {
            Self::Public(e) | Self::Private(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
