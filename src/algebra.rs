use std::ops;
use serde::{Serialize};

/// The Extended Euclidean Algorithm to calculate the multiplicate inverse of a
/// `FieldElement`.
pub fn xgcd(a: i128, b: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        /// Error handle with division by zero?
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_s, old_t, old_r)
}
#[derive(Default, Copy, Clone, Debug)]
struct FieldElement {
    value: u128,
    field: Field
}

impl ops::Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        self.field.add(self, rhs)
    }
}

impl ops::Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        self.field.multiply(self, rhs)
    }
}

impl ops::Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        self.field.subtract(self, rhs)
    }
}

impl ops::Div for FieldElement {
    type Output = FieldElement;
    
    fn div(self, rhs: Self) -> Self::Output {
        self.field.divide(self, rhs)
    }
}

impl ops::Neg for FieldElement {
    type Output = FieldElement;
    
    fn neg(self) -> Self::Output {
        self.field.negate(self)
    }
}

impl ops::BitXor for FieldElement {
    type Output = FieldElement;
    
    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Serialize for FieldElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        todo!()
    }
}

impl FieldElement {
    pub fn new(value: u128, field: Field) -> Self{
        Self { value, field}
    }

    fn inverse(&self) -> FieldElement {
        self.field.inverse(*self)
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct Field {
    p: u128
}

impl Field {
    pub fn zero() -> FieldElement {
        todo!()
    }

    pub fn one() -> FieldElement {
        todo!()
    }

    pub fn add(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        todo!()
    }

    pub fn multiply(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        todo!()
    }

    pub fn subtract(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        todo!()
    }

    pub fn divide(&self, a: FieldElement, b: FieldElement) -> FieldElement {
        todo!()
    }

    pub fn negate(&self, operand: FieldElement) -> FieldElement {
        todo!()
    }

    pub fn inverse(&self, operand: FieldElement) -> FieldElement {
        todo!()
    }
}