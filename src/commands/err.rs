//! Declare an error type

use thiserror::Error;

/// An error originated by a command.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Relay fingerprint not found: {0}")]
    RelayNotFound(String),
    #[error("Invalid filter: {0}")]
    InvalidFilter(String),
    #[error("Undecodable fingerprint: {0}")]
    UndecodableFingerprint(String),
    #[error("Unrecognized filter: {0}")]
    UnrecognizedFilter(String),
    #[error("Wrong fingerprint length: {0}")]
    WrongFingerprintLength(String),
}
