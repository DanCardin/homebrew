pub mod density;
use density::{Density, SpecificGravity};

pub mod gravity;
pub use gravity::*;

pub mod brix;
pub use brix::*;

// pub trait Abv {
//     fn abv(original: impl Into<Density>, r#final: impl Into<Density>) -> f32;
// }

struct StandardFormula;
struct AlternateFormula;
struct CutaiaFormula;

impl StandardFormula {
    fn abv(
        original: impl Into<Density<SpecificGravity>>,
        r#final: impl Into<Density<SpecificGravity>>,
    ) -> f32 {
        let original = original.into();
        let r#final = r#final.into();
        let abv = (original.to_f32() - r#final.to_f32()) * 131.25;
        abv
    }
}

// impl Abv for AlternateFormula {
//     fn abv(original: impl Into<Density>, r#final: impl Into<Density>) -> f32 {
//         let original = original.into();
//         let r#final = r#final.into();
//         let a = 76.08 * (original - r#final);
//         let b = 1.775 - original;
//         let c = r#final / 0.794;
//         let abv = (a / b) * c;
//         abv.into()
//     }
// }

// impl Abv for CutaiaFormula {
//     fn abv(original: impl Into<Density>, r#final: impl Into<Density>) -> f32 {
//         let original = Brix::from(original.into());
//         let r#final = Brix::from(r#final.into());

//         let abw = (0.372 + 0.00357 * original) * (original - r#final);
//         let a = 0.00001308
//             + 0.003868 * r#final
//             + 0.00001275 * r#final.pow(2)
//             + 0.000000063 * r#final.pow(3)
//             + 1.;
//         let abv = abw * (a / 0.7909);
//         abv.into()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_standard() {
        let og = Density::<SpecificGravity>::new(1.050);
        let fg = Density::<SpecificGravity>::new(1.010);
        let result = StandardFormula::abv(og, fg);
        assert_eq!(result, 5.249995);
    }

    // #[test]
    // fn it_calculates_alternative() {
    //     let og = Density::from(1.050);
    //     let fg = Density::from(1.010);
    //     let result = AlternateFormula::abv(og, fg);
    //     assert_eq!(result, 5.3394055);
    // }

    // #[test]
    // fn it_calculates_cutaia() {
    //     let og = Density::from(1.050);
    //     let fg = Density::from(1.010);
    //     let result = CutaiaFormula::abv(og, fg);
    //     assert_eq!(result, 5.2231045);
    // }
}
