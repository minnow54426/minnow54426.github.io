use crate::fields::FieldWrapper;
use ark_ff::PrimeField;

#[derive(Clone, Debug)]
pub struct Polynomial<F: PrimeField> {
    pub coeffs: Vec<FieldWrapper<F>>,
}

impl<F: PrimeField> Polynomial<F> {
    pub fn new(coeffs: Vec<FieldWrapper<F>>) -> Self {
        Self { coeffs }
    }

    pub fn evaluate(&self, x: &FieldWrapper<F>) -> FieldWrapper<F> {
        let mut result = FieldWrapper::zero();
        let mut x_pow = FieldWrapper::one();

        for coeff in &self.coeffs {
            result = result + coeff.clone() * x_pow.clone();
            x_pow = x_pow.clone() * x.clone();
        }

        result
    }

    pub fn degree(&self) -> usize {
        if self.coeffs.is_empty() {
            return 0;
        }
        self.coeffs.len() - 1
    }

    /// Checks if this polynomial is the zero polynomial
    pub fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|c| c.value.is_zero())
    }
}

impl<F: PrimeField> std::ops::Mul for Polynomial<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return Self::new(vec![]);
        }

        let deg_self = self.degree();
        let deg_rhs = rhs.degree();
        let mut result_coeffs = vec![FieldWrapper::<F>::zero(); deg_self + deg_rhs + 1];

        for (i, a_coeff) in self.coeffs.iter().enumerate() {
            for (j, b_coeff) in rhs.coeffs.iter().enumerate() {
                result_coeffs[i + j] =
                    result_coeffs[i + j].clone() + a_coeff.clone() * b_coeff.clone();
            }
        }

        // Remove trailing zero coefficients
        while result_coeffs.len() > 1 && result_coeffs.last().unwrap().value.is_zero() {
            result_coeffs.pop();
        }

        Self::new(result_coeffs)
    }
}

impl<F: PrimeField> std::ops::Sub for Polynomial<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let max_len = self.coeffs.len().max(rhs.coeffs.len());
        let mut result_coeffs = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a_coeff = self
                .coeffs
                .get(i)
                .cloned()
                .unwrap_or_else(FieldWrapper::zero);
            let b_coeff = rhs
                .coeffs
                .get(i)
                .cloned()
                .unwrap_or_else(FieldWrapper::zero);
            result_coeffs.push(a_coeff - b_coeff);
        }

        // Remove trailing zero coefficients
        while result_coeffs.len() > 1 && result_coeffs.last().unwrap().value.is_zero() {
            result_coeffs.pop();
        }

        Self::new(result_coeffs)
    }
}

impl<F: PrimeField> std::ops::Add for Polynomial<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let max_len = self.coeffs.len().max(rhs.coeffs.len());
        let mut result_coeffs = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a_coeff = self
                .coeffs
                .get(i)
                .cloned()
                .unwrap_or_else(FieldWrapper::zero);
            let b_coeff = rhs
                .coeffs
                .get(i)
                .cloned()
                .unwrap_or_else(FieldWrapper::zero);
            result_coeffs.push(a_coeff + b_coeff);
        }

        // Remove trailing zero coefficients
        while result_coeffs.len() > 1 && result_coeffs.last().unwrap().value.is_zero() {
            result_coeffs.pop();
        }

        Self::new(result_coeffs)
    }
}
