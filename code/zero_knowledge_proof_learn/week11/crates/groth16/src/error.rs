use thiserror::Error;

/// Errors that can occur during Groth16 operations
#[derive(Error, Debug)]
pub enum Groth16Error {
    #[error("Mismatched polynomial vector lengths: a={0}, b={1}, c={2}")]
    MismatchedPolynomials(usize, usize, usize),

    #[error("Empty polynomial vectors provided")]
    EmptyPolynomials,

    #[error("Invalid number of inputs: {0}")]
    InvalidInputs(usize),

    #[error("Polynomial evaluation error: {0}")]
    EvaluationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid witness length: expected {expected}, got {actual}")]
    InvalidWitnessLength { expected: usize, actual: usize },

    #[error("Division error: {0}")]
    DivisionError(String),

    #[error("QAP error: {0}")]
    QapError(String),
}
