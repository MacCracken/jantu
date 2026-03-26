use alloc::string::String;
use thiserror::Error;

/// Errors produced by jantu operations.
///
/// # Examples
///
/// ```
/// use jantu::JantuError;
///
/// let err = JantuError::InvalidDrive("negative hunger".into());
/// assert!(err.to_string().contains("negative hunger"));
/// ```
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JantuError {
    /// An invalid or unrecognized behavior was requested.
    #[error("invalid behavior: {0}")]
    InvalidBehavior(String),
    /// A drive value was out of range or otherwise invalid.
    #[error("invalid drive: {0}")]
    InvalidDrive(String),
    /// A social state transition was invalid.
    #[error("invalid social state: {0}")]
    InvalidSocialState(String),
    /// A numerical computation produced an invalid result.
    #[error("computation error: {0}")]
    ComputationError(String),
}

/// Convenience alias for `core::result::Result<T, JantuError>`.
pub type Result<T> = core::result::Result<T, JantuError>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error_display() {
        let e = JantuError::InvalidBehavior("unknown instinct".into());
        assert!(e.to_string().contains("unknown instinct"));
    }
}
