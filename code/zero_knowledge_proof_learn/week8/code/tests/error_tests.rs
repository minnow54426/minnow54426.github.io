use zk_groth16_snark::{Error, ErrorKind, SetupError, VerifyError};

#[test]
fn test_error_display() {
    let err = Error::Setup(SetupError::ParametersAlreadyExist);
    assert!(err.to_string().contains("Setup"));
}

#[test]
fn test_error_kind() {
    let err = Error::Verify(VerifyError::InvalidProof);
    assert!(matches!(err.kind(), ErrorKind::InvalidProof));
}
