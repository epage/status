use std::error::Error;
use std::fmt;

use crate::AlarmKind;
use crate::Context;

type StdError = dyn Error + 'static;

/// Ad hoc error.
///
/// Goals:
/// - Easy crate inter-op while maintaining programmatic processing.
/// - User-friendly without losing helpful debug information.
///
/// Note: this is optimized for the happy-path.  When failing frequently inside of an inner loop,
/// consider implementing `Error` on your `AlarmKind`.
///
/// # Example
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
///     type Context = alarm::NoContext;
/// }
/// type Alarm = alarm::Alarm<ErrorKind>;
/// type Result<T, E = Alarm> = std::result::Result<T, E>;
///
/// pub fn read_file() -> Result<()> {
///     return ErrorKind::Read.into_err();
/// }
/// ```
#[derive(Debug)]
pub struct Alarm<K: AlarmKind>(Box<AlarmDetails<K>>);

#[derive(Debug)]
struct AlarmDetails<K: AlarmKind> {
    kind: K,
    source: Source,
    data: K::Context,
}

impl<K: AlarmKind> Alarm<K> {
    /// Create a new error object from the error kind.
    pub fn new(kind: K) -> Self {
        Self(Box::new(AlarmDetails {
            kind,
            source: Source::Empty,
            data: Default::default(),
        }))
    }

    /// Add an internal error.
    pub fn with_internal<E: Error + 'static>(mut self, error: E) -> Self {
        self.0.source = Source::Private(Box::new(error));
        self
    }

    /// Add a public error.
    pub fn with_source<E: Error + 'static>(mut self, error: E) -> Self {
        self.0.source = Source::Public(Box::new(error));
        self
    }

    /// Programmatic identifier for which error occurred.
    pub fn kind(&self) -> K {
        self.0.kind
    }

    /// An iterator for the chain of sources.
    pub fn sources(&self) -> Chain {
        Chain {
            next: self.source(),
        }
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
    pub fn into_internal(self) -> InternalAlarm<K> {
        InternalAlarm(self)
    }

    /// Convenience for returning an error.
    pub fn into_err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl<K: AlarmKind> fmt::Display for Alarm<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.0.kind)?;
        if !self.0.data.is_empty() {
            writeln!(f)?;
            writeln!(f, "{}", self.0.data)?;
        }
        Ok(())
    }
}

impl<K: AlarmKind> std::ops::Deref for Alarm<K> {
    type Target = K::Context;

    fn deref(&self) -> &Self::Target {
        &self.0.data
    }
}

impl<K: AlarmKind> std::ops::DerefMut for Alarm<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.data
    }
}

impl<K: AlarmKind> Error for Alarm<K> {
    fn cause(&self) -> Option<&dyn Error> {
        self.0.source.public()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source.public()
    }
}

// impl From<AlarmKind> is waiting on specialization

// impl From<Error> is waiting on specialization

/// View of the error, exposing implementation details.
#[derive(Debug)]
pub struct InternalAlarm<K: AlarmKind>(Alarm<K>);

impl<K: AlarmKind> InternalAlarm<K> {
    /// An iterator for the chain of sources.
    pub fn sources(&self) -> Chain {
        Chain {
            next: self.source(),
        }
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

impl<K: AlarmKind> fmt::Display for InternalAlarm<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl<K: AlarmKind> Error for InternalAlarm<K> {
    fn cause(&self) -> Option<&dyn Error> {
        (self.0).0.source.any()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        (self.0).0.source.any()
    }
}

#[derive(Debug)]
pub struct Chain<'a> {
    next: Option<&'a StdError>,
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a StdError;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take()?;
        self.next = next.source();
        Some(next)
    }
}

#[derive(Debug)]
enum Source {
    Public(Box<StdError>),
    Private(Box<StdError>),
    Empty,
}

impl Source {
    fn public(&self) -> Option<&StdError> {
        match self {
            Self::Public(e) => Some(e.as_ref()),
            _ => None,
        }
    }

    fn any(&self) -> Option<&StdError> {
        match self {
            Self::Public(e) | Self::Private(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
