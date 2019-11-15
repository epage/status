use std::error::Error;
use std::fmt;

use crate::AlarmKind;

type StdError = dyn Error + 'static;

/// Ad hoc error context.
///
/// Goals:
/// - Easy crate inter-op while maintaining programmatic processing.
/// - User-friendly without losing helpful debug information.
/// - Easily extend with context at each level of your crate
///
/// Note: this is optimized for the happy-path.  When failing frequently inside of an inner loop,
/// consider implementing `Error` on `AlarmKind`.
#[derive(Debug)]
pub struct Alarm<K: AlarmKind>(Box<AlarmDetails<K>>);

#[derive(Debug)]
struct AlarmDetails<K: AlarmKind> {
    kind: K,
    source: Source,
}

impl<K: AlarmKind> Alarm<K> {
    /// Create a new error object from the error kind.
    pub fn new(kind: K) -> Self {
        Self(Box::new(AlarmDetails {
            kind,
            source: Source::Empty,
        }))
    }

    /// Add in an internal error.
    pub fn with_internal<E: Error + 'static>(mut self, error: E) -> Self {
        self.0.source = Source::Internal(Box::new(error));
        self
    }

    /// Add a public error.
    pub fn with_context<E: Error + 'static>(mut self, error: E) -> Self {
        self.0.source = Source::Context(Box::new(error));
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
    pub fn into_internal(self) -> Internal<K> {
        Internal(self)
    }

    /// Convenience for returning an error.
    pub fn into_err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl<K: AlarmKind> fmt::Display for Alarm<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.kind)
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
pub struct Internal<K: AlarmKind>(Alarm<K>);

impl<K: AlarmKind> Internal<K> {
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

impl<K: AlarmKind> fmt::Display for Internal<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (self.0).0.kind)
    }
}

impl<K: AlarmKind> Error for Internal<K> {
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
    Context(Box<StdError>),
    Internal(Box<StdError>),
    Empty,
}

impl Source {
    fn public(&self) -> Option<&StdError> {
        match self {
            Self::Context(e) => Some(e.as_ref()),
            _ => None,
        }
    }

    fn any(&self) -> Option<&StdError> {
        match self {
            Self::Context(e) | Self::Internal(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
