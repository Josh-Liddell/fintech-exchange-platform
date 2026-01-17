/// An application-specific error type
#[derive(Debug, PartialEq, Eq)]
pub enum ApplicationError {
    AccountNotFound(String),
    AccountUnderFunded(String, u64),
    AccountOverFunded(String, u64),
}
