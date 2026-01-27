use thiserror::Error;

/// Circuit type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CircuitType {
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "membership")]
    Membership,
    #[serde(rename = "privacy")]
    Privacy,
}

impl std::fmt::Display for CircuitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitType::Identity => write!(f, "identity"),
            CircuitType::Membership => write!(f, "membership"),
            CircuitType::Privacy => write!(f, "privacy"),
        }
    }
}

impl std::str::FromStr for CircuitType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "identity" => Ok(CircuitType::Identity),
            "membership" => Ok(CircuitType::Membership),
            "privacy" => Ok(CircuitType::Privacy),
            _ => Err(format!("Unknown circuit type: {}", s)),
        }
    }
}

/// CLI error types
#[derive(Error, Debug)]
pub enum CliError {
    #[error("File not found: {0}")]
    FileNotFound(std::path::PathBuf),

    #[error("Invalid JSON in {0}: {1}")]
    InvalidJson(String, String),

    #[error("Circuit type mismatch: expected {expected}, found {found}")]
    CircuitMismatch { expected: CircuitType, found: CircuitType },

    #[error("Unsupported circuit type: {0}")]
    UnsupportedCircuit(CircuitType),

    #[error("Proof generation failed: {0}")]
    ProveFailed(String),

    #[error("Verification failed: {0}")]
    VerifyFailed(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, CliError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CliError::FileNotFound("test.json".into());
        assert!(err.to_string().contains("test.json"));
    }

    #[test]
    fn test_circuit_mismatch() {
        let err = CliError::CircuitMismatch {
            expected: CircuitType::Identity,
            found: CircuitType::Membership,
        };
        assert!(err.to_string().contains("identity"));
        assert!(err.to_string().contains("membership"));
    }
}
