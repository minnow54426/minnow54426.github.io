#[cfg(test)]
mod tests {
    use crate::fields::FieldWrapper;
    use ark_bn254::Fq;

    #[test]
    fn test_field_wrapper_creation() {
        let field = FieldWrapper::<Fq>::from(5u64);
        assert_eq!(field.value, Fq::from(5u64));
    }

    #[test]
    fn test_field_wrapper_arithmetic() {
        let a = FieldWrapper::<Fq>::from(5u64);
        let b = FieldWrapper::<Fq>::from(3u64);
        let sum = a + b;
        assert_eq!(sum.value, Fq::from(8u64));
    }
}
