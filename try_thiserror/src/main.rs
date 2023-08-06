use std::io;
// use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

// not use thiserror
// #[derive(Debug)]
// pub enum DataStoreError {
//     Disconnect(io::Error),
//     Redaction(String),
//     InvalidHeader { expected: String, found: String },
//     Unknown,
// }

// impl fmt::Display for DataStoreError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Disconnect(_) => write!(f, "data store disconnected"),
//             Self::Redaction(s) => write!(f, "the data for key `{s}` is not available"),
//             Self::InvalidHeader { expected, found }  => write!(f, "invalid header (expected {expected:?}, found {found:?})"),
//             Self::Unknown => write!(f, "unknown data store error"),
//         }
//     }
// }

// impl From<io::Error> for DataStoreError {
//     fn from(err: io::Error) -> Self {
//         DataStoreError::Disconnect(err)
//     }
// }

fn connect_data_store() -> Result<(), io::Error> {
    Err(io::Error::from(io::ErrorKind::ConnectionAborted))
}

fn get_user() -> Result<(), DataStoreError> {
    connect_data_store()?;
    // do something
    Ok(())
}

fn main() {
    let err = get_user().unwrap_err();
    assert_eq!(err.to_string(), "data store disconnected")
}
