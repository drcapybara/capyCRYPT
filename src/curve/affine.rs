use super::{field::field_element::FieldElement, extended_edwards::ExtendedPoint};

#[derive(Debug)]
pub struct AffinePoint {
    pub(crate) x: FieldElement,
    pub(crate) y: FieldElement,
}

impl AffinePoint {
    pub fn identity() -> AffinePoint {
        AffinePoint {
            x: FieldElement::zero(),
            y: FieldElement::one(),
        }
    }
    pub fn to_extended(&self) -> ExtendedPoint {
        ExtendedPoint {
            X: self.x,
            Y: self.y,
            Z: FieldElement::one(),
            T: self.x * self.y,
        }
    }
}