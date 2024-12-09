use crate::{TokenIter, TokenTree};
use std::sync::Arc;

/// Result type for parsing.
pub type Result<T> = std::result::Result<T, Error>;

// To keep the Error passing simple and allocation free for the common cases we define these
// common cases plus adding the generic case as dyn boxed error.
/// Actual kind of an error.
#[derive(Clone)]
enum ErrorKind {
    /// A no error state that can be upgraded by later errors.
    NoError,
    /// Trying to parse `expected` but found `found`.
    UnexpectedToken {
        /// type name of what was expected
        expected: &'static str,
        /// the token that was found
        found: TokenTree,
    },
    /// Trying to parse `expected` but found the end of the input.
    UnexpectedEnd {
        /// type name of what was expected
        expected: &'static str,
    },
    /// Something else failed which can be fully formatted as `String`.
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
    kind: ErrorKind,
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

    /// Create a `Result<T>::Err(Error{ kind: ErrorKind::UnexpectedToken }` error.
    #[allow(clippy::missing_errors_doc)]
    pub fn unexpected_token<T>(pos: impl TokenCount, found: TokenTree) -> Result<T> {
        Err(Error {
            kind: ErrorKind::UnexpectedToken {
                expected: std::any::type_name::<T>(),
                found,
            },
            pos: pos.token_count(),
        })
    }

    /// Create a `Result<T>::Err(Error{ kind: ErrorKind::UnexpectedEnd }` error.
    #[allow(clippy::missing_errors_doc)]
    pub fn unexpected_end<T>() -> Result<T> {
        Err(Error {
            kind: ErrorKind::UnexpectedEnd {
                expected: std::any::type_name::<T>(),
            },
            pos: usize::MAX,
        })
    }

    /// Either `UnexpectedToken` or `UnexpectedEnd` depending if token is `Some`.
    #[allow(clippy::missing_errors_doc)]
    pub fn unexpected_token_or_end<T>(pos: impl TokenCount, token: Option<TokenTree>) -> Result<T> {
        match token {
            Some(token) => Error::unexpected_token(pos, token),
            None => Error::unexpected_end(),
        }
    }

    /// Create a `Result<T>::Err(Error{ kind: ErrorKind::Other }` error.
    #[allow(clippy::missing_errors_doc)]
    pub fn other<T>(pos: impl TokenCount, reason: String) -> Result<T> {
        Err(Error {
            kind: ErrorKind::Other { reason },
            pos: pos.token_count(),
        })
    }

    /// Create a `Error::Dynamic` error.
    pub fn dynamic(pos: impl TokenCount, err: impl std::error::Error + 'static) -> Self {
        Error {
            kind: ErrorKind::Dynamic(Arc::new(err)),
            pos: pos.token_count(),
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::NoError => {
                write!(f, "NoError")
            }
            ErrorKind::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "Unexpected token: expected {expected}, found {found:?} at {:?}",
                    found.span().start()
                )
            }
            ErrorKind::UnexpectedEnd { expected } => {
                write!(f, "Unexpected end of input: expected {expected}")
            }
            ErrorKind::Other { reason } => {
                write!(f, "{reason}")
            }
            ErrorKind::Dynamic(err) => {
                write!(f, "Error: {err}")
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
            ErrorKind::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "Unexpected token: expected {expected}, found {found:?} at {:?}",
                    found.span().start()
                )
            }
            ErrorKind::UnexpectedEnd { expected } => {
                write!(f, "Unexpected end of input: expected {expected}")
            }
            ErrorKind::Other { reason } => {
                write!(f, "{reason}")
            }
            ErrorKind::Dynamic(err) => {
                write!(f, "Error: {err}")
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

// defining for && allows us to pass a &TokenIter by reference when it is still needed
// later. Otherwise it would need to be reborrowed '&*iter' which is less ergonomic.
impl TokenCount for &&TokenIter<'_> {
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

impl TokenCount for &&mut TokenIter<'_> {
    #[inline]
    fn token_count(self) -> usize {
        self.counter()
    }
}
