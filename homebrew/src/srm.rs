use maplit::hashmap;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[allow(dead_code)]
pub static SRM_TO_HEX: Lazy<HashMap<u8, &'static str>> = Lazy::new(|| {
    hashmap! {
        1 => "#FFE699",
        2 => "#FFD878",
        3 => "#FFCA5A",
        4 => "#FFBF42",
        5 => "#FBB123",
        6 => "#F8A600",
        7 => "#F39C00",
        8 => "#EA8F00",
        9 => "#E58500",
        10 => "#DE7C00",
        11 => "#D77200",
        12 => "#CF6900",
        13 => "#CB6200",
        14 => "#C35900",
        15 => "#BB5100",
        16 => "#B54C00",
        17 => "#B04500",
        18 => "#A63E00",
        19 => "#A13700",
        20 => "#9B3200",
        21 => "#952D00",
        22 => "#8E2900",
        23 => "#882300",
        24 => "#821E00",
        25 => "#7B1A00",
        26 => "#771900",
        27 => "#701400",
        28 => "#6A0E00",
        29 => "#660D00",
        30 => "#5E0B00",
        31 => "#5A0A02",
        32 => "#600903",
        33 => "#520907",
        34 => "#4C0505",
        35 => "#470606",
        36 => "#440607",
        37 => "#3F0708",
        38 => "#3B0607",
        39 => "#3A070B",
        40 => "#36080A",
    }
});

pub struct SRM(u8);

impl SRM {
    pub fn to_hex(&self) -> &'static str {
        return SRM_TO_HEX.get(&self.0).unwrap_or(&"#000000");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_srm() {
        let srm = SRM(1).to_hex();
        assert_eq!(srm, "#FFE699");
    }

    #[test]
    fn invalid_srm() {
        let srm = SRM(50).to_hex();
        assert_eq!(srm, "#000000");
    }
}
