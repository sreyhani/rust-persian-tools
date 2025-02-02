use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum VerifyLegalIdError {
    #[error("invalid len legal id should be 11 chars")]
    InvalidLength,

    #[error("invalid digit")]
    InvalidDigit,

    #[error("invalid legal id")]
    Invalid,

    #[error("invalid legal id")]
    InvalidChecksum,

    #[error("you should not see this error. make a issue on our github please.")]
    InternalError,
}
