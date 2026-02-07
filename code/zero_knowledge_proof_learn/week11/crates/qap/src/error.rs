use thiserror::Error;

/// Errors that can occur during QAP operations
#[derive(Error, Debug)]
pub enum QapError {
    #[error("No constraints provided")]
    EmptyConstraints,

    #[error("Need at least 2 constraints for interpolation")]
    InsufficientConstraints,

    #[error("No points provided for interpolation")]
    EmptyPoints,

    #[error("Duplicate x-value in interpolation points: {0}")]
    DuplicateX(String),

    #[error("Mismatched lengths: witness has {0} elements but polynomials have {1}")]
    MismatchedLengths(usize, usize),

    #[error("Division by zero polynomial")]
    DivisionByZero,
}
