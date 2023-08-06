use std::io;
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
