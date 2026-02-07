#[cfg(test)]
mod tests {
    use crate::pairing::PairingGroup;
    use ark_bn254::{Bn254, G1Affine, G1Projective as G1, G2Affine, G2Projective as G2};
    use ark_ec::pairing::Pairing;
    use ark_ec::AffineRepr;

    #[test]
    fn test_pairing_bilinearity() {
        // Get generators using the Affine generator method
        let g1 = G1::from(G1Affine::generator());
        let g2 = G2::from(G2Affine::generator());

        // Test basic pairing equation: e(g1, g2) == e(g1, g2)
        assert!(PairingGroup::verify_pairing_equation(&g1, &g2, &g1, &g2));

        // Test bilinearity property with scalar multiplication
        type ScalarField = <Bn254 as Pairing>::ScalarField;

        let a = ScalarField::from(2u64);
        let b = ScalarField::from(3u64);

        let g1_a = g1 * a;
        let g2_b = g2 * b;

        // Due to bilinearity: e(g1^a, g2^b) should be equal to itself
        assert!(PairingGroup::verify_pairing_equation(
            &g1_a, &g2_b, &g1_a, &g2_b
        ));

        // Also test that different pairings are NOT equal
        let g1_different = g1 * ScalarField::from(5u64);
        assert!(!PairingGroup::verify_pairing_equation(
            &g1,
            &g2,
            &g1_different,
            &g2
        ));
    }
}
