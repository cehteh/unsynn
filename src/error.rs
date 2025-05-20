use crate::{TokenIter, TokenStream};
use std::sync::Arc;

/// Result type for parsing.
pub type Result<T> = std::result::Result<T, Error>;

// To keep the Error passing simple and allocation free for the common cases we define these
// common cases plus adding the generic case as dyn boxed error.
/// Actual kind of an error.
#[derive(Clone)]
pub enum ErrorKind {
    /// A no error state that can be upgraded by later errors.
    NoError,
    /// Parser failed.
    UnexpectedToken,
    /// Something else failed which can be formatted as `String`.
    Other {
        /// explanation what failed
        reason: String,
    },
    /// Any other error.
    Dynamic(Arc<dyn std::error::Error>),
}

/// Error type for parsing.
#[must_use]
#[derive(Clone)]
pub struct Error {
    /// Kind of the error.
    pub kind: ErrorKind,
    /// type name of what was expected
    expected: &'static str,
    /// refines type name for complex parsers
    refined: Option<&'static str>,
    /// Iterator starting at the error
    at: Option<<TokenStream as IntoIterator>::IntoIter>,
    // ShadowCountedIter position where it happened
    // on disjunct parsers we use this to determine which error to keep
    pos: usize,
}

impl Error {
    /// Create a `ErrorKind::NoError` error.
    #[allow(clippy::missing_errors_doc)]
    pub const fn no_error() -> Self {
        Error {
            kind: ErrorKind::NoError,
            expected: "<NoError>",
            refined: None,
            at: None,
            pos: 0,
        }
    }

    /// Upgrade an error to one with greater pos value.
    #[allow(clippy::missing_errors_doc)]
    pub fn upgrade<T>(&mut self, r: Result<T>) -> Result<T> {
        if let Err(other) = &r {
            if matches!(self.kind, ErrorKind::NoError) || other.pos > self.pos {
                *self = other.clone();
            }
        }
        r
    }

    /// Set the position of the error.
    ///
    /// Sometimes the position of the error is not known at the time of creation. This allows
    /// to adjust it later.
    pub fn set_pos(&mut self, pos: impl TokenCount) {
        self.pos = pos.token_count();
    }

    /// Get the position of the error.
    #[must_use]
    pub const fn pos(&self) -> usize {
        self.pos
    }

    /// Create a `Result<T>::Err(Error{ kind: ErrorKind::Unexpected })` error.
    #[allow(clippy::missing_errors_doc)]
    pub fn unexpected_token<T>(at: &TokenIter) -> Result<T> {
        Err(Error {
            kind: ErrorKind::UnexpectedToken,
            expected: std::any::type_name::<T>(),
            refined: None,
            at: Some(at.clone().into_inner_iter()),
            pos: at.token_count(),
        })
    }

    /// Create a `Result<T>::Err(Error{ kind: ErrorKind::UnexpectedEnd })` error.
    #[allow(clippy::missing_errors_doc)]
    pub fn unexpected_end<T>() -> Result<T> {
        Err(Error {
            kind: ErrorKind::UnexpectedToken,
            expected: std::any::type_name::<T>(),
            refined: None,
            at: None,
            pos: usize::MAX,
        })
    }

    /// Create a `Result<T>::Err(Error{ kind: ErrorKind::Other })` error.
    #[allow(clippy::missing_errors_doc)]
    pub fn other<T>(at: &TokenIter, reason: String) -> Result<T> {
        Err(Error {
            kind: ErrorKind::Other { reason },
            expected: std::any::type_name::<T>(),
            refined: None,
            at: Some(at.clone().into_inner_iter()),
            pos: at.token_count(),
        })
    }

    /// Create a `Error::Dynamic` error.
    pub fn dynamic<T>(at: &TokenIter, err: impl std::error::Error + 'static) -> Self {
        Error {
            kind: ErrorKind::Dynamic(Arc::new(err)),
            expected: std::any::type_name::<T>(),
            refined: None,
            at: Some(at.clone().into_inner_iter()),
            pos: at.token_count(),
        }
    }
}

impl std::error::Error for Error {}

/// Pretty printer for Options, either prints None or T without the enclosing Some.
struct OptionPP<'a, T>(&'a Option<T>);

impl<T: std::fmt::Debug> std::fmt::Debug for OptionPP<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{value:?}"),
            None => write!(f, "None"),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for OptionPP<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(value) => write!(f, "{value}"),
            None => write!(f, "None"),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::NoError => {
                write!(f, "NoError")
            }
            ErrorKind::UnexpectedToken => {
                write!(
                    f,
                    "Unexpected token: expected {}, found {:?} at {:?}",
                    self.refined.unwrap_or(self.expected),
                    OptionPP(&self.at),
                    OptionPP(
                        &self
                            .at
                            .clone()
                            .map(|mut f| f.next().map(|s| s.span().start()))
                    )
                )
            }
            ErrorKind::Other { reason } => {
                write!(
                    f,
                    "Parser failed: expected {}, because {reason}, found {:?} at {:?}",
                    self.refined.unwrap_or(self.expected),
                    OptionPP(&self.at),
                    OptionPP(
                        &self
                            .at
                            .clone()
                            .map(|mut f| f.next().map(|s| s.span().start()))
                    )
                )
            }
            ErrorKind::Dynamic(err) => {
                write!(
                    f,
                    "Parser failed: expected {}, because {err}, found {:?} at {:?}",
                    self.refined.unwrap_or(self.expected),
                    OptionPP(&self.at),
                    OptionPP(
                        &self
                            .at
                            .clone()
                            .map(|mut f| f.next().map(|s| s.span().start()))
                    )
                )
            }
        }
    }
}

impl std::fmt::Display for Error {
    #[cfg_attr(test, mutants::skip)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::NoError => {
                write!(f, "NoError")
            }
            ErrorKind::UnexpectedToken => {
                write!(
                    f,
                    "Unexpected token: expected {}, found {:?} at {:?}",
                    self.refined.unwrap_or(self.expected),
                    OptionPP(&self.at),
                    OptionPP(
                        &self
                            .at
                            .clone()
                            .map(|mut f| f.next().map(|s| s.span().start()))
                    )
                )
            }
            ErrorKind::Other { reason } => {
                write!(
                    f,
                    "Parser failed: expected {}, because {reason}, found {:?} at {:?}",
                    self.refined.unwrap_or(self.expected),
                    OptionPP(&self.at),
                    OptionPP(
                        &self
                            .at
                            .clone()
                            .map(|mut f| f.next().map(|s| s.span().start()))
                    )
                )
            }
            ErrorKind::Dynamic(err) => {
                write!(
                    f,
                    "Parser failed: expected {}, because {err}, found {:?} at {:?}",
                    self.refined.unwrap_or(self.expected),
                    OptionPP(&self.at),
                    OptionPP(
                        &self
                            .at
                            .clone()
                            .map(|mut f| f.next().map(|s| s.span().start()))
                    )
                )
            }
        }
    }
}

/// We track the position of the error by counting tokens. This trait is implemented for
/// references to shadow counted `TokenIter`, and `usize`. The later allows to pass in a
/// position directly or use `usize::MAX` in case no position data is available (which will
/// make this error the be the final one when upgrading).
pub trait TokenCount {
    /// Get the position of the token iterator.
    fn token_count(self) -> usize;
}

// Allows passing a usize directly.
impl TokenCount for usize {
    #[inline]
    fn token_count(self) -> usize {
        self
    }
}

impl TokenCount for &TokenIter<'_> {
    #[inline]
    fn token_count(self) -> usize {
        self.counter()
    }
}

impl TokenCount for &mut TokenIter<'_> {
    #[inline]
    fn token_count(self) -> usize {
        self.counter()
    }
}

// implementing for &&mut allows us to pass a &mut TokenIter by reference when it is still needed
// later. Otherwise it would need to be reborrow '&mut *iter' which is less ergonomic.
impl TokenCount for &&mut TokenIter<'_> {
    #[inline]
    fn token_count(self) -> usize {
        self.counter()
    }
}
