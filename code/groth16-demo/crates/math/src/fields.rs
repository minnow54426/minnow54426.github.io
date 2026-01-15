use ark_ff::PrimeField;
use serde::{Deserialize, Serialize};

/// Wrapper around arkworks field elements for type safety
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldWrapper<F: PrimeField> {
    pub value: F,
}

impl<F: PrimeField> FieldWrapper<F> {
    pub fn from(value: impl Into<F>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn zero() -> Self {
        Self { value: F::zero() }
    }

    pub fn one() -> Self {
        Self { value: F::one() }
    }
}

impl<F: PrimeField> std::ops::Add for FieldWrapper<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl<F: PrimeField> std::ops::Mul for FieldWrapper<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
        }
    }
}

impl<F: PrimeField> std::ops::Sub for FieldWrapper<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}
