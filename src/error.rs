use std::error::Error;
use std::fmt;
use std::path::PathBuf;

/// Error that could happen during connecting to LND
///
/// This error may be returned by the `connect()` function if connecting failed.
/// It is currently opaque because it's unclear how the variants will look long-term.
/// Thus you probably only want to display it.
#[derive(Debug)]
pub struct ConnectError {
    internal: InternalConnectError,
}

impl From<InternalConnectError> for ConnectError {
    fn from(value: InternalConnectError) -> Self {
        ConnectError { internal: value }
    }
}

#[derive(Debug)]
pub(crate) enum InternalConnectError {
    Tonic(tonic::transport::Error),
    ReadFile {
        file: PathBuf,
        error: std::io::Error,
    },
    ParseCert {
        file: PathBuf,
        error: Box<dyn Error + Send + Sync + 'static>,
    },
    InvalidAddress {
        address: String,
        error: Box<dyn Error + Send + Sync + 'static>,
    },
    Other(Box<dyn Error>),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use InternalConnectError::*;

        match &self.internal {
            Tonic(error) => write!(f, "{:?}", error),
            ReadFile { file, .. } => write!(f, "failed to read file {}", file.display()),
            ParseCert { file, .. } => write!(f, "failed to parse certificate {}", file.display()),
            InvalidAddress { address, .. } => write!(f, "invalid address {}", address),
            Other(err) => write!(f, "unknown error: {}", err),
        }
    }
}

impl Error for ConnectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use InternalConnectError::*;

        match &self.internal {
            Tonic(error) => Some(error),
            ReadFile { error, .. } => Some(error),
            ParseCert { error, .. } => Some(&**error),
            InvalidAddress { error, .. } => Some(&**error),
            Other(err) => Some(&**err),
        }
    }
}

impl From<tonic::transport::Error> for ConnectError {
    fn from(value: tonic::transport::Error) -> Self {
        Self {
            internal: InternalConnectError::Tonic(value),
        }
    }
}