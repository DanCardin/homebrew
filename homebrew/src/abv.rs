pub mod gravity;
pub use gravity::*;

pub mod brix;
pub use brix::*;

pub enum AbvFormula {
    Standard,
    Alternative,
    Cutaia,
}

impl AbvFormula {
    pub fn calculate(&self, original: Gravity, final_: Gravity) -> f32 {
        match self {
            Self::Standard => Self::calculate_standard(original, final_),
            Self::Alternative => Self::calculate_alternative(original, final_),
            Self::Cutaia => Self::calculate_cutaia(original, final_),
        }
    }

    pub fn calculate_standard(original: Gravity, final_: Gravity) -> f32 {
        let abv = (original - final_) * 131.25;
        abv.into()
    }

    pub fn calculate_alternative(original: Gravity, final_: Gravity) -> f32 {
        let a = 76.08 * (original - final_);
        let b = 1.775 - original;
        let c = final_ / 0.794;
        let abv = (a / b) * c;
        abv.into()
    }

    pub fn calculate_cutaia(original: Gravity, final_: Gravity) -> f32 {
        let original = Brix::from(original);
        let final_ = Brix::from(final_);

        let abw = (0.372 + 0.00357 * original) * (original - final_);
        let a = 0.00001308
            + 0.003868 * final_
            + 0.00001275 * final_.pow(2)
            + 0.000000063 * final_.pow(3)
            + 1.;
        let abv = abw * (a / 0.7909);
        abv.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_standard() {
        let og = Gravity::from(1.050);
        let fg = Gravity::from(1.010);
        let result = AbvFormula::Standard.calculate(og, fg);
        assert_eq!(result, 5.249995);
    }

    #[test]
    fn it_calculates_alternative() {
        let og = Gravity::from(1.050);
        let fg = Gravity::from(1.010);
        let result = AbvFormula::Alternative.calculate(og, fg);
        assert_eq!(result, 5.3394055);
    }

    #[test]
    fn it_calculates_cutaia() {
        let og = Gravity::from(1.050);
        let fg = Gravity::from(1.010);
        let result = AbvFormula::Cutaia.calculate(og, fg);
        assert_eq!(result, 5.2231045);
    }
}
