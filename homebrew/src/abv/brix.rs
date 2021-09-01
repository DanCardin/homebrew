use crate::abv::Gravity;

// Degrees Brix (°Bx) is the sugar content of an aqueous solution.
#[derive(Copy, Clone, Debug)]
pub struct Brix(f32);

impl Brix {
    pub fn to_gravity(&self) -> Gravity {
        let brix = self.0;
        let gravity = (brix / (258.6 - ((brix / 258.2) * 227.1))) + 1.;
        Gravity::from(gravity)
    }

    pub fn with_wort_correction_factory(&self, factor: f32) -> Self {
        let correction = (factor - 1.) * self.0;
        Self(self.0 - correction)
    }

    pub fn pow(&self, exp: i32) -> Self {
        Self(self.0.powi(exp))
    }
}

impl std::ops::Div<Brix> for f32 {
    type Output = Brix;

    fn div(self, rhs: Brix) -> Self::Output {
        Brix(self / rhs.0)
    }
}

impl std::ops::Div<f32> for Brix {
    type Output = Brix;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl std::ops::Div<Brix> for Brix {
    type Output = Brix;

    fn div(self, rhs: Self) -> Self::Output {
        Brix(self.0 / rhs.0)
    }
}

impl std::ops::Mul<Brix> for Brix {
    type Output = Brix;

    fn mul(self, rhs: Self) -> Self::Output {
        Brix(self.0 * rhs.0)
    }
}

impl std::ops::Mul<Brix> for f32 {
    type Output = Brix;

    fn mul(self, rhs: Brix) -> Self::Output {
        Brix(self * rhs.0)
    }
}

impl std::ops::Mul<f32> for Brix {
    type Output = Brix;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Sub<f32> for Brix {
    type Output = Brix;

    fn sub(self, rhs: f32) -> Self::Output {
        Brix(self.0 - rhs)
    }
}

impl std::ops::Sub<Brix> for f32 {
    type Output = Brix;

    fn sub(self, rhs: Brix) -> Self::Output {
        Brix(self - rhs.0)
    }
}

impl std::ops::Sub<Brix> for Brix {
    type Output = Brix;

    fn sub(self, rhs: Brix) -> Self::Output {
        Brix(self.0 - rhs.0)
    }
}

impl std::ops::Add<f32> for Brix {
    type Output = Brix;

    fn add(self, rhs: f32) -> Self::Output {
        Brix(self.0 + rhs)
    }
}

impl std::ops::Add<Brix> for f32 {
    type Output = Brix;

    fn add(self, rhs: Brix) -> Self::Output {
        Brix(self + rhs.0)
    }
}

impl std::ops::Add<Brix> for Brix {
    type Output = Brix;

    fn add(self, rhs: Brix) -> Self::Output {
        Brix(self.0 + rhs.0)
    }
}

impl std::convert::From<f32> for Brix {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl std::convert::From<Brix> for f32 {
    fn from(value: Brix) -> Self {
        value.0
    }
}

impl std::convert::From<Gravity> for Brix {
    fn from(value: Gravity) -> Self {
        let gravity = f32::from(value);
        let brix = ((182.4601 * gravity - 775.6821) * gravity + 1262.7794) * gravity - 669.5622;
        Brix(brix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_from_gravity() {
        let og = Gravity::from(1.092);
        let result = Brix::from(og);
        assert_eq!(f32::from(result), 22.0141);
    }
}
