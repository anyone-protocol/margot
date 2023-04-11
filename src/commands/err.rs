//! Declare an error type

use thiserror::Error;

use tor_netdoc::types::policy::PolicyError;

/// An error originated by a command.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid filter: {0}")]
    InvalidFilter(String),
    #[error("Undecodable fingerprint: {0}")]
    UndecodableFingerprint(String),
    #[error("Unrecognized filter: {0}")]
    UnrecognizedFilter(String),
    #[error("Wrong fingerprint length: {0}")]
    WrongFingerprintLength(String),
    #[error("Policy error: {0}")]
    WrongPolicy(#[from] PolicyError),
    #[error("IO error: {0}")]
    WrongIO(#[from] std::io::Error),
}
