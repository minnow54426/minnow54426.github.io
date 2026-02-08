use ark_bn254::{G1Affine, G2Affine};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Helper to serialize a single arkworks type to bytes
fn serialize_to_bytes<T>(value: &T) -> Vec<u8>
where
    T: CanonicalSerialize,
{
    let mut bytes = Vec::new();
    value.serialize_compressed(&mut bytes).unwrap();
    bytes
}

/// Helper to deserialize a single arkworks type from bytes
fn deserialize_from_bytes<T>(bytes: &[u8]) -> T
where
    T: CanonicalDeserialize,
{
    T::deserialize_compressed(bytes).unwrap()
}

/// Helper to serialize a vector of arkworks types to bytes
fn serialize_vec_to_bytes<T>(values: &[T]) -> Vec<u8>
where
    T: CanonicalSerialize,
{
    let mut bytes = Vec::new();
    for value in values {
        value.serialize_compressed(&mut bytes).unwrap();
    }
    bytes
}

/// Helper to deserialize a vector of arkworks types from bytes
fn deserialize_vec_from_bytes<T>(bytes: &[u8]) -> Vec<T>
where
    T: CanonicalDeserialize,
{
    let mut values = Vec::new();
    let mut offset = 0;
    while offset < bytes.len() {
        let remaining = &bytes[offset..];
        let value = T::deserialize_compressed(remaining).unwrap();
        offset += bytes.len() - remaining.len();
        values.push(value);
    }
    values
}

/// Proving key for Groth16
///
/// The proving key contains all encrypted elements needed to generate proofs.
/// It must be kept secret by the prover (though in Groth16, it's public knowledge).
#[derive(Clone, Debug)]
pub struct ProvingKey {
    /// α·G₁ (used in proof A component)
    pub alpha_g1: G1Affine,

    /// β·G₁ (used in proof A component)
    pub beta_g1: G1Affine,

    /// β·G₂ (used in verification)
    pub beta_g2: G2Affine,

    /// δ·G₁ (used in proof C component)
    pub delta_g1: G1Affine,

    /// δ·G₂ (used in verification)
    pub delta_g2: G2Affine,

    /// Encrypted A-polynomials: [α·Aᵢ(τ)·G₁] for i=0..m
    pub a_query: Vec<G1Affine>,

    /// Encrypted B-polynomials in G1: [β·Bᵢ(τ)·G₁] for i=0..m
    pub b_g1_query: Vec<G1Affine>,

    /// Encrypted B-polynomials in G2: [β·Bᵢ(τ)·G₂] for i=0..m
    pub b_g2_query: Vec<G2Affine>,

    /// Encrypted C-polynomials: [β·Cᵢ(τ)·G₁] for i=0..m
    pub c_query: Vec<G1Affine>,

    /// Encrypted division polynomials: [Hᵢ(τ)·G₁] for i=0..n-2
    pub h_query: Vec<G1Affine>,
}

/// Serializable representation of ProvingKey
#[derive(Serialize, Deserialize)]
struct ProvingKeyRepr {
    #[serde(with = "serde_bytes")]
    alpha_g1: Vec<u8>,
    #[serde(with = "serde_bytes")]
    beta_g1: Vec<u8>,
    #[serde(with = "serde_bytes")]
    beta_g2: Vec<u8>,
    #[serde(with = "serde_bytes")]
    delta_g1: Vec<u8>,
    #[serde(with = "serde_bytes")]
    delta_g2: Vec<u8>,
    #[serde(with = "serde_bytes")]
    a_query: Vec<u8>,
    #[serde(with = "serde_bytes")]
    b_g1_query: Vec<u8>,
    #[serde(with = "serde_bytes")]
    b_g2_query: Vec<u8>,
    #[serde(with = "serde_bytes")]
    c_query: Vec<u8>,
    #[serde(with = "serde_bytes")]
    h_query: Vec<u8>,
}

impl From<&ProvingKey> for ProvingKeyRepr {
    fn from(pk: &ProvingKey) -> Self {
        ProvingKeyRepr {
            alpha_g1: serialize_to_bytes(&pk.alpha_g1),
            beta_g1: serialize_to_bytes(&pk.beta_g1),
            beta_g2: serialize_to_bytes(&pk.beta_g2),
            delta_g1: serialize_to_bytes(&pk.delta_g1),
            delta_g2: serialize_to_bytes(&pk.delta_g2),
            a_query: serialize_vec_to_bytes(&pk.a_query),
            b_g1_query: serialize_vec_to_bytes(&pk.b_g1_query),
            b_g2_query: serialize_vec_to_bytes(&pk.b_g2_query),
            c_query: serialize_vec_to_bytes(&pk.c_query),
            h_query: serialize_vec_to_bytes(&pk.h_query),
        }
    }
}

impl From<&ProvingKeyRepr> for ProvingKey {
    fn from(repr: &ProvingKeyRepr) -> Self {
        ProvingKey {
            alpha_g1: deserialize_from_bytes(&repr.alpha_g1),
            beta_g1: deserialize_from_bytes(&repr.beta_g1),
            beta_g2: deserialize_from_bytes(&repr.beta_g2),
            delta_g1: deserialize_from_bytes(&repr.delta_g1),
            delta_g2: deserialize_from_bytes(&repr.delta_g2),
            a_query: deserialize_vec_from_bytes(&repr.a_query),
            b_g1_query: deserialize_vec_from_bytes(&repr.b_g1_query),
            b_g2_query: deserialize_vec_from_bytes(&repr.b_g2_query),
            c_query: deserialize_vec_from_bytes(&repr.c_query),
            h_query: deserialize_vec_from_bytes(&repr.h_query),
        }
    }
}

impl Serialize for ProvingKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ProvingKeyRepr::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ProvingKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = ProvingKeyRepr::deserialize(deserializer)?;
        Ok(ProvingKey::from(&repr))
    }
}

/// Verification key for Groth16
///
/// The verification key contains the public elements needed to verify proofs.
/// It can be shared publicly.
#[derive(Clone, Debug)]
pub struct VerificationKey {
    /// α·G₁ (part of verification equation)
    pub alpha_g1: G1Affine,

    /// β·G₂ (part of verification equation)
    pub beta_g2: G2Affine,

    /// γ·G₂ (base for public input encryption)
    pub gamma_g2: G2Affine,

    /// δ·G₂ (base for proof C encryption)
    pub delta_g2: G2Affine,

    /// Public input encryption: [β·G₁, β·U₁(τ)·G₁ + α·H₁(τ)·G₁, ...]
    /// The first element IC[0] = β·G₁ is for the constant 1,
    /// followed by elements for each public input
    pub ic: Vec<G1Affine>,
}

/// Serializable representation of VerificationKey
#[derive(Serialize, Deserialize)]
struct VerificationKeyRepr {
    #[serde(with = "serde_bytes")]
    alpha_g1: Vec<u8>,
    #[serde(with = "serde_bytes")]
    beta_g2: Vec<u8>,
    #[serde(with = "serde_bytes")]
    gamma_g2: Vec<u8>,
    #[serde(with = "serde_bytes")]
    delta_g2: Vec<u8>,
    #[serde(with = "serde_bytes")]
    ic: Vec<u8>,
}

impl From<&VerificationKey> for VerificationKeyRepr {
    fn from(vk: &VerificationKey) -> Self {
        VerificationKeyRepr {
            alpha_g1: serialize_to_bytes(&vk.alpha_g1),
            beta_g2: serialize_to_bytes(&vk.beta_g2),
            gamma_g2: serialize_to_bytes(&vk.gamma_g2),
            delta_g2: serialize_to_bytes(&vk.delta_g2),
            ic: serialize_vec_to_bytes(&vk.ic),
        }
    }
}

impl From<&VerificationKeyRepr> for VerificationKey {
    fn from(repr: &VerificationKeyRepr) -> Self {
        VerificationKey {
            alpha_g1: deserialize_from_bytes(&repr.alpha_g1),
            beta_g2: deserialize_from_bytes(&repr.beta_g2),
            gamma_g2: deserialize_from_bytes(&repr.gamma_g2),
            delta_g2: deserialize_from_bytes(&repr.delta_g2),
            ic: deserialize_vec_from_bytes(&repr.ic),
        }
    }
}

impl Serialize for VerificationKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        VerificationKeyRepr::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for VerificationKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = VerificationKeyRepr::deserialize(deserializer)?;
        Ok(VerificationKey::from(&repr))
    }
}
