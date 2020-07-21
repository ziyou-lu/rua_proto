use std::{io, error, fmt};
use std::fmt::Display;

/// An enum of all error kinds.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ErrorKind {
    /// will read bytes over than left bytes
    NoLeftSpaceError,
    /// the buffer over max
    BufferOverMaxError,
    /// The type not match
    TypeNotMatchError,
    /// the buffer can't parse the right data
    ParseError,
    /// miss the major data
    MissingError,
    /// string format must be utf-8
    StringFormatError,
    /// This kind is returned if the redis error is one that is
    /// not native to the system.  This is usually the case if
    /// the cause is another error.
    IoError,
    /// An extension error.  This is an error created by the server
    /// that is not directly understood by the library.
    ExtensionError,
}

#[derive(Debug)]
enum ErrorRepr {
    WithDescription(ErrorKind, &'static str),
    WithDescriptionAndDetail(ErrorKind, &'static str, String),
    ExtensionError(String, String),
    IoError(io::Error),
}

/// Represents a redis error.  For the most part you should be using
/// the Error trait to interact with this rather than the actual
/// struct.
pub struct RpError {
    repr: ErrorRepr,
}

/// Library generic result type.
pub type RpResult<T> = Result<T, RpError>;


impl PartialEq for RpError {
    fn eq(&self, other: &RpError) -> bool {
        match (&self.repr, &other.repr) {
            (&ErrorRepr::WithDescription(kind_a, _), &ErrorRepr::WithDescription(kind_b, _)) => {
                kind_a == kind_b
            }
            (&ErrorRepr::WithDescriptionAndDetail(kind_a, _, _),
             &ErrorRepr::WithDescriptionAndDetail(kind_b, _, _)) => kind_a == kind_b,
            (&ErrorRepr::ExtensionError(ref a, _), &ErrorRepr::ExtensionError(ref b, _)) => {
                *a == *b
            }
            _ => false,
        }
    }
}

impl From<io::Error> for RpError {
    fn from(err: io::Error) -> RpError {
        RpError { repr: ErrorRepr::IoError(err) }
    }
}


impl From<(ErrorKind, &'static str)> for RpError {
    fn from((kind, desc): (ErrorKind, &'static str)) -> RpError {
        RpError { repr: ErrorRepr::WithDescription(kind, desc) }
    }
}

impl From<(ErrorKind, &'static str, String)> for RpError {
    fn from((kind, desc, detail): (ErrorKind, &'static str, String)) -> RpError {
        RpError { repr: ErrorRepr::WithDescriptionAndDetail(kind, desc, detail) }
    }
}

impl error::Error for RpError {
    fn description(&self) -> &str {
        match self.repr {
            ErrorRepr::WithDescription(_, desc) => desc,
            ErrorRepr::WithDescriptionAndDetail(_, desc, _) => desc,
            ErrorRepr::ExtensionError(_, _) => "extension error",
            ErrorRepr::IoError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match self.repr {
            ErrorRepr::IoError(ref err) => Some(err as &dyn error::Error),
            _ => None,
        }
    }
}

impl fmt::Display for RpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.repr {
            ErrorRepr::WithDescription(_, desc) => desc.fmt(f),
            ErrorRepr::WithDescriptionAndDetail(_, desc, ref detail) => {
                (desc.fmt(f))?;
                (f.write_str(": "))?;
                detail.fmt(f)
            }
            ErrorRepr::ExtensionError(ref code, ref detail) => {
                (code.fmt(f))?;
                (f.write_str(": "))?;
                detail.fmt(f)
            }
            ErrorRepr::IoError(ref err) => err.fmt(f),
        }
    }
}

impl fmt::Debug for RpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

/// Indicates a general failure in the library.
impl RpError {
    /// Returns the kind of the error.
    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            ErrorRepr::WithDescription(kind, _) => kind,
            ErrorRepr::WithDescriptionAndDetail(kind, _, _) => kind,
            ErrorRepr::ExtensionError(_, _) => ErrorKind::ExtensionError,
            ErrorRepr::IoError(_) => ErrorKind::IoError,
        }
    }

    /// Returns the name of the error category for display purposes.
    pub fn category(&self) -> &str {
        match self.kind() {
            ErrorKind::NoLeftSpaceError => "no left space error",
            ErrorKind::BufferOverMaxError => "buffer over max error",
            ErrorKind::TypeNotMatchError => "type not match error",
            ErrorKind::ParseError => "parse error",
            ErrorKind::MissingError => "missing error",
            ErrorKind::StringFormatError => "string format error",
            ErrorKind::IoError => "I/O error",
            ErrorKind::ExtensionError => "extension error",
        }
    }

    /// Indicates that this failure is an IO failure.
    pub fn is_io_error(&self) -> bool {
        match self.kind() {
            ErrorKind::IoError => true,
            _ => false,
        }
    }

    /// Returns the extension error code
    pub fn extension_error_code(&self) -> Option<&str> {
        match self.repr {
            ErrorRepr::ExtensionError(ref code, _) => Some(&code),
            _ => None,
        }
    }

    /// Returns the extension error detail
    pub fn extension_error_detail(&self) -> Option<&str> {
        match self.repr {
            ErrorRepr::ExtensionError(_, ref detail) => Some(&detail),
            ErrorRepr::WithDescriptionAndDetail(_, _, ref detail) => Some(&detail),
            _ => None,
        }
    }
}

pub fn make_extension_error(code: &str, detail: Option<&str>) -> RpError {
    RpError {
        repr: ErrorRepr::ExtensionError(code.to_string(),
                                        match detail {
                                            Some(x) => x.to_string(),
                                            None => {
                                                "Unknown extension error encountered".to_string()
                                            }
                                        }),
    }
}