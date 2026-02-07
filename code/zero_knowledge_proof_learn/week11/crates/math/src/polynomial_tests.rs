#[cfg(test)]
mod tests {
    use crate::fields::FieldWrapper;
    use crate::polynomial::Polynomial;
    use ark_bn254::Fq;

    #[test]
    fn test_polynomial_evaluation() {
        // p(x) = 2x + 3
        let coeffs = vec![
            FieldWrapper::<Fq>::from(3u64), // constant term
            FieldWrapper::<Fq>::from(2u64), // x term
        ];
        let poly = Polynomial::new(coeffs);

        // p(5) = 2*5 + 3 = 13
        let x = FieldWrapper::<Fq>::from(5u64);
        let result = poly.evaluate(&x);
        assert_eq!(result.value, Fq::from(13u64));
    }
}
