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
    Connect {
        address: String,
        error: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    ReadFile {
        file: PathBuf,
        error: std::io::Error,
    },
    ParseCert {
        file: PathBuf,
        error: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    InvalidAddress {
        address: String,
        error: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    Other(Box<dyn std::error::Error>),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use InternalConnectError::*;

        match &self.internal {
            Connect { address, error } => write!(f, "failed to connect to {} {}", address, error),
            ReadFile { file, .. } => write!(f, "failed to read file {}", file.display()),
            ParseCert { file, .. } => write!(f, "failed to parse certificate {}", file.display()),
            InvalidAddress { address, .. } => write!(f, "invalid address {}", address),
            Other(err) => write!(f, "unknown error: {}", err),
        }
    }
}

impl std::error::Error for ConnectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use InternalConnectError::*;

        match &self.internal {
            Connect { error, .. } => Some(&**error),
            ReadFile { error, .. } => Some(error),
            ParseCert { error, .. } => Some(&**error),
            InvalidAddress { error, .. } => Some(&**error),
            Other(err) => Some(&**err),
        }
    }
}
