pub use proc_macro2::{Span, TokenTree};

// To keep the Error passing simple and allocation free for the common cases we define these
// common cases plus adding the generic case as dyn boxed error.
/// Error type for parsing.
pub enum Error {
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
    /// Something else failed when trying to parse `expected`.
    Other {
        /// explanation what failed
        reason: String,
    },
    /// Any other error.
    Boxed(Box<dyn std::error::Error>),
}

impl Error {
    /// Create a `Result<T>::Err(Error::UnexpectedToken)` error.
    #[allow(clippy::missing_errors_doc)]
    pub fn unexpected_token<T>(found: TokenTree) -> Result<T> {
        Err(Error::UnexpectedToken {
            expected: std::any::type_name::<T>(),
            found,
        })
    }

    /// Create a `Result<T>::Err(Error::UnexpectedEnd)` error.
    #[allow(clippy::missing_errors_doc)]
    pub fn unexpected_end<T>() -> Result<T> {
        Err(Error::UnexpectedEnd {
            expected: std::any::type_name::<T>(),
        })
    }

    /// Create a `Result<T>::Err(Error::Other)` error.
    #[allow(clippy::missing_errors_doc)]
    pub fn other<T>(reason: String) -> Result<T> {
        Err(Error::Other { reason })
    }

    /// Create a `Error::Boxed` error.
    pub fn boxed(err: impl std::error::Error + 'static) -> Self {
        Error::Boxed(Box::new(err))
    }
}

impl std::error::Error for Error {}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "Unexpected token: expected {expected}, found {found:?} at {:?}",
                    found.span().start()
                )
            }
            Error::UnexpectedEnd { expected } => {
                write!(f, "Unexpected end of input: expected {expected}")
            }
            Error::Other { reason } => {
                write!(f, "{reason}")
            }
            Error::Boxed(err) => {
                write!(f, "Error: {err}")
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "Unexpected token: expected {expected}, found {found:?} at {:?}",
                    found.span().start()
                )
            }
            Error::UnexpectedEnd { expected } => {
                write!(f, "Unexpected end of input: expected {expected}")
            }
            Error::Other { reason } => {
                write!(f, "{reason}")
            }
            Error::Boxed(err) => {
                write!(f, "Error: {err}")
            }
        }
    }
}

/// Result type for parsing.
pub type Result<T> = std::result::Result<T, Error>;
