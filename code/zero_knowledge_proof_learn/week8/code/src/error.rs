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
            // Circuit errors - all map to InvalidWitness
            Error::Circuit(CircuitError::Identity(_)) => ErrorKind::InvalidWitness,
            Error::Circuit(CircuitError::Membership(_)) => ErrorKind::InvalidWitness,
            Error::Circuit(CircuitError::Privacy(_)) => ErrorKind::InvalidWitness,
            Error::Circuit(CircuitError::SynthesisError(_)) => ErrorKind::ConstraintViolation,

            // Setup errors
            Error::Setup(SetupError::ParametersAlreadyExist) => ErrorKind::ParametersAlreadyExist,
            Error::Setup(SetupError::InsufficientEntropy) => ErrorKind::InsufficientEntropy,
            Error::Setup(SetupError::SetupFailed) => ErrorKind::SetupFailed,

            // Prove errors
            Error::Prove(ProveError::WitnessGenerationFailed) => ErrorKind::WitnessGenerationFailed,
            Error::Prove(ProveError::ProofCreationFailed) => ErrorKind::ProofCreationFailed,

            // Verify errors
            Error::Verify(VerifyError::InvalidProof) => ErrorKind::InvalidProof,
            Error::Verify(VerifyError::ProofVerificationFailed) => {
                ErrorKind::ProofVerificationFailed
            }
            Error::Verify(VerifyError::PublicInputsIncorrect) => ErrorKind::PublicInputsIncorrect,

            // Serialization errors
            Error::Serialization(SerializationError::DeserializationFailed) => {
                ErrorKind::DeserializationFailed
            }
            Error::Serialization(SerializationError::VersionMismatch) => ErrorKind::VersionMismatch,

            // IO errors map to Unknown
            Error::Io(_) => ErrorKind::Unknown,
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

    #[error("Constraint synthesis error: {0}")]
    SynthesisError(String),
}

impl From<ark_relations::r1cs::SynthesisError> for CircuitError {
    fn from(err: ark_relations::r1cs::SynthesisError) -> Self {
        CircuitError::SynthesisError(err.to_string())
    }
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
