use thiserror::Error;

/// Main error type for the library
#[derive(Error, Debug)]
pub enum Error {
    #[error("Circuit error: {0}")]
    Circuit(#[from] CircuitError),

    #[error("Setup error: {0}")]
    Setup(#[from] SetupError),

    #[error("Prove error: {0}")]
    Prove(#[from] ProveError),

    #[error("Verify error: {0}")]
    Verify(#[from] VerifyError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] SerializationError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::Circuit(CircuitError::Identity(_)) => ErrorKind::InvalidWitness,
            Error::Setup(SetupError::ParametersAlreadyExist) => ErrorKind::ParametersAlreadyExist,
            Error::Prove(ProveError::ProofCreationFailed) => ErrorKind::ProofCreationFailed,
            Error::Verify(VerifyError::InvalidProof) => ErrorKind::InvalidProof,
            _ => ErrorKind::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    InvalidWitness,
    ConstraintViolation,
    PublicInputMismatch,
    ParametersAlreadyExist,
    InsufficientEntropy,
    SetupFailed,
    WitnessGenerationFailed,
    ProofCreationFailed,
    InvalidProof,
    ProofVerificationFailed,
    PublicInputsIncorrect,
    DeserializationFailed,
    VersionMismatch,
    Unknown,
}

#[derive(Error, Debug)]
pub enum CircuitError {
    #[error("Identity circuit error: {0}")]
    Identity(#[from] IdentityError),

    #[error("Membership circuit error: {0}")]
    Membership(#[from] MembershipError),

    #[error("Privacy circuit error: {0}")]
    Privacy(#[from] PrivacyError),
}

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Parameters already exist")]
    ParametersAlreadyExist,

    #[error("Insufficient entropy")]
    InsufficientEntropy,

    #[error("Setup failed")]
    SetupFailed,
}

#[derive(Error, Debug)]
pub enum ProveError {
    #[error("Witness generation failed")]
    WitnessGenerationFailed,

    #[error("Proof creation failed")]
    ProofCreationFailed,
}

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Invalid proof")]
    InvalidProof,

    #[error("Proof verification failed")]
    ProofVerificationFailed,

    #[error("Public inputs incorrect")]
    PublicInputsIncorrect,
}

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("Deserialization failed")]
    DeserializationFailed,

    #[error("Version mismatch")]
    VersionMismatch,
}

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("Invalid preimage length")]
    InvalidPreimageLength,

    #[error("Hash mismatch")]
    HashMismatch,
}

#[derive(Error, Debug)]
pub enum MembershipError {
    #[error("Invalid path length")]
    InvalidPathLength,

    #[error("Root mismatch")]
    RootMismatch,

    #[error("Leaf not found")]
    LeafNotFound,
}

#[derive(Error, Debug)]
pub enum PrivacyError {
    #[error("Value out of range")]
    ValueOutOfRange,

    #[error("Invalid bit width")]
    InvalidBitWidth,
}

pub type Result<T> = std::result::Result<T, Error>;
