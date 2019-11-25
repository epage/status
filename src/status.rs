use std::error;
use std::fmt;

use crate::AdhocContext;
use crate::Chain;
use crate::Context;
use crate::InternalStatus;
use crate::Kind;
use crate::StdError;
use crate::StrictError;
use crate::Unkind;

/// A container for use in `Result<_, Status>`..
///
/// Goals:
/// - Easy crate inter-op while maintaining programmatic processing.
/// - User-friendly without losing helpful debug information.
///
/// Note: this is optimized for the happy-path.  When failing frequently inside of an inner loop,
/// consider using your [`Kind`] to convey your status.
///
/// # Example
///
/// ```rust
/// use status::Kind;
///
/// #[derive(Copy, Clone, Debug, derive_more::Display)]
/// enum ErrorKind {
///   #[display(fmt = "Failed to read file")]
///   Read,
///   #[display(fmt = "Failed to parse")]
///   Parse,
/// }
/// type Status = status::Status<ErrorKind>;
/// type Result<T, E = Status> = std::result::Result<T, E>;
///
/// fn read_file() -> Result<()> {
///     return ErrorKind::Read.into_err();
/// }
/// ```
#[derive(Debug)]
pub struct Status<K: Kind = Unkind, C: Context = AdhocContext> {
    pub(crate) inner: Box<StatusDetails<K, C>>,
}

#[derive(Debug)]
pub(crate) struct StatusDetails<K: Kind, C: Context> {
    pub(crate) kind: K,
    pub(crate) source: Source,
    pub(crate) data: C,
}

impl<K: Kind, C: Context> Status<K, C> {
    /// Create a container for the specified status [`Kind`].
    ///
    /// # Example
    ///
    /// ```
    /// fn read_file() -> Result<(), status::Status> {
    ///     return Err(status::Status::new("Failed to read file"));
    /// }
    /// ```
    pub fn new<U>(kind: U) -> Self
    where
        U: Into<K>,
    {
        Self {
            inner: Box::new(StatusDetails {
                kind: kind.into(),
                source: Source::Empty,
                data: Default::default(),
            }),
        }
    }

    /// Add a public error.
    #[cfg(feature = "send_sync")]
    pub fn with_source<E>(mut self, error: E) -> Self
    where
        E: error::Error + Send + Sync + 'static,
    {
        self.inner.source = Source::Public(Box::new(error));
        self
    }
    /// Add a public error.
    #[cfg(not(feature = "send_sync"))]
    pub fn with_source<E>(mut self, error: E) -> Self
    where
        E: error::Error + 'static,
    {
        self.inner.source = Source::Public(Box::new(error));
        self
    }

    #[cfg(feature = "send_sync")]
    /// Add an internal error.
    pub fn with_internal<E>(mut self, error: E) -> Self
    where
        E: error::Error + Send + Sync + 'static,
    {
        self.inner.source = Source::Private(Box::new(error));
        self
    }
    #[cfg(not(feature = "send_sync"))]
    /// Add an internal error.
    pub fn with_internal<E>(mut self, error: E) -> Self
    where
        E: error::Error + 'static,
    {
        self.inner.source = Source::Private(Box::new(error));
        self
    }

    /// Extend the [`Context`].
    pub fn context_with<F>(mut self, context: F) -> Self
    where
        F: Fn(C) -> C,
    {
        let mut data: C = Default::default();
        std::mem::swap(&mut data, &mut self.inner.data);
        let mut data = context(data);
        std::mem::swap(&mut data, &mut self.inner.data);
        self
    }

    /// Access the [`Context`] for programmatic usage.
    pub fn context(&self) -> &C {
        &self.inner.data
    }

    /// Programmatic identifier for which error occurred.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use status::Kind;
    /// #
    /// #[derive(Copy, Clone, Debug, derive_more::Display)]
    /// enum ErrorKind {
    ///   #[display(fmt = "Failed to read file")]
    ///   Read,
    ///   #[display(fmt = "Failed to parse")]
    ///   Parse,
    /// }
    /// # type Status = status::Status<ErrorKind>;
    /// # type Result<T, E = Status> = std::result::Result<T, E>;
    ///
    /// fn read_file() -> Result<()> {
    /// #     return ErrorKind::Read.into_err();
    /// }
    ///
    /// fn process_file() -> Result<()> {
    ///     match read_file() {
    ///         Ok(_) => Ok(()),
    ///         Err(e) => match e.kind() {
    ///           ErrorKind::Read => Err(e),
    ///           ErrorKind::Parse => Ok(()),
    ///         }
    ///     }
    /// }
    /// ```
    pub fn kind(&self) -> K {
        self.inner.kind
    }

    /// An iterator for the chain of sources.
    ///
    /// When debugging, to display internal sources, run [`Status::into_internal`].
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
    pub fn sources(&self) -> Chain {
        Chain::new(error::Error::source(self))
    }

    /// The lowest level cause of this error &mdash; this error's cause's
    /// cause's cause etc.
    ///
    /// The root cause is the last error in the iterator produced by
    /// [`sources()`][Status::sources].
    ///
    /// # Example
    ///
    /// ```
    /// use status::Status;
    /// use std::io;
    ///
    /// pub fn underlying_io_error_kind(error: &Status) -> Option<io::ErrorKind> {
    ///     let cause = error.root_source()?;
    ///     if let Some(io_error) = cause.downcast_ref::<io::Error>() {
    ///         return Some(io_error.kind());
    ///     }
    ///     None
    /// }
    /// ```
    pub fn root_source(&self) -> Option<&StdError> {
        self.sources().last()
    }

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
    pub fn into_internal(self) -> InternalStatus<K, C> {
        InternalStatus::new(self)
    }

    /// Convenience for returning an error.
    pub fn into_err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl<K: Kind, C: Context> fmt::Display for Status<K, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.inner.kind)?;
        if !self.inner.data.is_empty() {
            writeln!(f)?;
            writeln!(f, "{}", self.inner.data)?;
        }
        Ok(())
    }
}

impl<K: Kind, C: Context> std::ops::Deref for Status<K, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.inner.data
    }
}

impl<K: Kind, C: Context> std::ops::DerefMut for Status<K, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner.data
    }
}

impl<K: Kind, C: Context> error::Error for Status<K, C> {
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
    Public(Box<StrictError>),
    Private(Box<StrictError>),
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

#[cfg(test)]
mod test {
    use super::*;

    use static_assertions::*;

    #[test]
    fn source() {
        assert_impl_all!(Source: fmt::Debug);
        #[cfg(feature = "send_sync")]
        assert_impl_all!(Source: Send, Sync);
    }

    #[test]
    fn status() {
        assert_impl_all!(Status: fmt::Debug, fmt::Display, error::Error);
        #[cfg(feature = "send_sync")]
        assert_impl_all!(Status: Send, Sync);
    }
}
