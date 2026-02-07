#[cfg(test)]
mod tests {
    use crate::constraint::R1CSConstraint;
    use ark_bn254::Fq;
    use groth16_math::fields::FieldWrapper;

    #[test]
    fn test_witness_satisfaction() {
        // Constraint: a * b = c
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
        constraint.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));
        constraint.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));

        let witness = vec![
            FieldWrapper::<Fq>::from(3u64),  // a = 3
            FieldWrapper::<Fq>::from(4u64),  // b = 4
            FieldWrapper::<Fq>::from(12u64), // c = 12
        ];

        // Should satisfy: 3 * 4 = 12 ✓
        assert!(constraint.is_satisfied(&witness));
    }

    #[test]
    fn test_witness_violation() {
        let mut constraint = R1CSConstraint::<Fq>::new();
        constraint.add_a_variable(0, FieldWrapper::<Fq>::from(1u64));
        constraint.add_b_variable(1, FieldWrapper::<Fq>::from(1u64));
        constraint.add_c_variable(2, FieldWrapper::<Fq>::from(1u64));

        let witness = vec![
            FieldWrapper::<Fq>::from(3u64),  // a = 3
            FieldWrapper::<Fq>::from(4u64),  // b = 4
            FieldWrapper::<Fq>::from(13u64), // c = 13 (wrong!)
        ];

        // Should not satisfy: 3 * 4 != 13 ✗
        assert!(!constraint.is_satisfied(&witness));
    }
}
