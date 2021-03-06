use crate::abv::{AbvFormula, Brix};

/// Specific gravitiy is the relative density of a substance compared to water.
#[derive(Copy, Clone, Debug)]
pub struct Gravity(f32);

impl Gravity {
    pub fn to_brix(&self) -> Brix {
        Brix::from(*self)
    }

    pub fn to(&self, gravity: Gravity) -> GravityDifferential {
        GravityDifferential::new(*self, gravity)
    }

    pub fn pow(&self, exp: i32) -> Self {
        Self(self.0.powi(exp))
    }
}

impl std::ops::Div<Gravity> for f32 {
    type Output = Gravity;

    fn div(self, rhs: Gravity) -> Self::Output {
        Gravity(self / rhs.0)
    }
}

impl std::ops::Div<f32> for Gravity {
    type Output = Gravity;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl std::ops::Div<Gravity> for Gravity {
    type Output = Gravity;

    fn div(self, rhs: Self) -> Self::Output {
        Gravity(self.0 / rhs.0)
    }
}

impl std::ops::Mul<Gravity> for Gravity {
    type Output = Gravity;

    fn mul(self, rhs: Self) -> Self::Output {
        Gravity(self.0 * rhs.0)
    }
}

impl std::ops::Mul<Gravity> for f32 {
    type Output = Gravity;

    fn mul(self, rhs: Gravity) -> Self::Output {
        Gravity(self * rhs.0)
    }
}

impl std::ops::Mul<f32> for Gravity {
    type Output = Gravity;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Sub<f32> for Gravity {
    type Output = Gravity;

    fn sub(self, rhs: f32) -> Self::Output {
        Gravity(self.0 - rhs)
    }
}

impl std::ops::Sub<Gravity> for f32 {
    type Output = Gravity;

    fn sub(self, rhs: Gravity) -> Self::Output {
        Gravity(self - rhs.0)
    }
}

impl std::ops::Sub<Gravity> for Gravity {
    type Output = Gravity;

    fn sub(self, rhs: Gravity) -> Self::Output {
        Gravity(self.0 - rhs.0)
    }
}

impl std::ops::Add<f32> for Gravity {
    type Output = Gravity;

    fn add(self, rhs: f32) -> Self::Output {
        Gravity(self.0 + rhs)
    }
}

impl std::ops::Add<Gravity> for f32 {
    type Output = Gravity;

    fn add(self, rhs: Gravity) -> Self::Output {
        Gravity(self + rhs.0)
    }
}

impl std::ops::Add<Gravity> for Gravity {
    type Output = Gravity;

    fn add(self, rhs: Gravity) -> Self::Output {
        Gravity(self.0 + rhs.0)
    }
}

impl std::convert::From<f32> for Gravity {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl std::convert::From<Gravity> for f32 {
    fn from(value: Gravity) -> Self {
        value.0
    }
}

pub struct GravityDifferential {
    original: Gravity,
    final_: Gravity,
}

impl GravityDifferential {
    pub fn new(original: Gravity, final_: Gravity) -> Self {
        Self { original, final_ }
    }

    pub fn abv(&self, formula: AbvFormula) -> f32 {
        formula.calculate(self.original, self.final_)
    }
}

impl Default for GravityDifferential {
    fn default() -> Self {
        Self {
            original: Gravity(1.),
            final_: Gravity(1.),
        }
    }
}

impl From<(f32, f32)> for GravityDifferential {
    fn from(gravities: (f32, f32)) -> Self {
        Self::new(Gravity(gravities.0), Gravity(gravities.1))
    }
}
