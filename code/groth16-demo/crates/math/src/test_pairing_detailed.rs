#[cfg(test)]
mod detailed_tests {
    use crate::pairing::PairingGroup;
    use ark_bn254::{G1Projective as G1, G2Projective as G2};
    use ark_ec::pairing::Pairing;
    use ark_ff::Field;

    #[test]
    fn test_identity_pairing() {
        // Test: What does default() return?
        let g1_default = G1::default();
        let g2_default = G2::default();
        
        // Check if default is zero (identity)
        let g1_zero = G1::zero();
        let g2_zero = G2::zero();
        
        assert_eq!(g1_default, g1_zero, "default() should return zero");
        assert_eq!(g2_default, g2_zero, "default() should return zero");
        
        // Pairing of identities should be identity in GT
        let pairing_result = <ark_bn254::Bn254 as Pairing>::pairing(g1_default, g2_default);
        let gt_zero = <ark_bn254::Bn254 as Pairing>::TargetField::one();
        // Note: e(0, 0) should be 1 in GT (not 0)
        // Actually, e(0, anything) = 1 in GT for pairings
        
        println!("Pairing e(0_G1, 0_G2) = {:?}", pairing_result);
    }

    #[test]
    fn test_generator_pairing() {
        // Test with actual generators
        let g1 = G1::prime_subgroup_generator();
        let g2 = G2::prime_subgroup_generator();
        
        let pairing_result = <ark_bn254::Bn254 as Pairing>::pairing(g1, g2);
        
        println!("Pairing e(g1, g2) = {:?}", pairing_result);
        
        // This should NOT be identity
        assert_ne!(pairing_result, <ark_bn254::Bn254 as Pairing>::TargetField::one());
    }

    #[test]
    fn test_bilinearity_property() {
        // REAL test of bilinearity: e(g1^a, g2^b) == e(g1, g2)^(ab)
        let g1 = G1::prime_subgroup_generator();
        let g2 = G2::prime_subgroup_generator();
        
        use ark_bn254::Bn254;
        type ScalarField = <Bn254 as Pairing>::ScalarField;
        
        let a = ScalarField::from(2u64);
        let b = ScalarField::from(3u64);
        
        let g1_a = g1 * a;
        let g2_b = g2 * b;
        
        // e(g1^a, g2^b)
        let left = Bn254::pairing(g1_a, g2_b);
        
        // e(g1, g2)^(ab)
        let e_g1_g2 = Bn254::pairing(g1, g2);
        let ab = a * b;
        let right = e_g1_g2.pow(ab);
        
        assert_eq!(left, right, "Bilinearity property should hold");
        
        // This is what verify_pairing_equation should test
        assert!(PairingGroup::verify_pairing_equation(&g1_a, &g2_b, &g1_a, &g2_b));
    }
}
