use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JantuError {
    #[error("invalid behavior: {0}")]
    InvalidBehavior(String),
    #[error("invalid drive: {0}")]
    InvalidDrive(String),
    #[error("invalid social state: {0}")]
    InvalidSocialState(String),
    #[error("computation error: {0}")]
    ComputationError(String),
}

pub type Result<T> = std::result::Result<T, JantuError>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error_display() {
        let e = JantuError::InvalidBehavior("unknown instinct".into());
        assert!(e.to_string().contains("unknown instinct"));
    }
}
