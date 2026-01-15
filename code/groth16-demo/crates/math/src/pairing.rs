use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;

pub struct PairingGroup;

impl PairingGroup {
    pub fn verify_pairing_equation(
        a: &<Bn254 as Pairing>::G1,
        b: &<Bn254 as Pairing>::G2,
        c: &<Bn254 as Pairing>::G1,
        d: &<Bn254 as Pairing>::G2,
    ) -> bool {
        // e(a, b) == e(c, d)
        let left = Bn254::pairing(*a, *b);
        let right = Bn254::pairing(*c, *d);
        left == right
    }
}
