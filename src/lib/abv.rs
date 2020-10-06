pub struct Abv {
    original_brix: f32,
    final_brix: f32,
}

impl Abv {
    fn gravity_to_brix(gravity: f32) -> f32 {
        ((182.4601 * gravity - 775.6821) * gravity + 1262.7794) * gravity - 669.5622
    }

    fn brix_to_gravity(brix: f32) -> f32 {
        (brix / (258.6 - ((brix / 258.2) * 227.1))) + 1.
    }
}

impl Abv {
    pub fn new() -> Self {
        Self {
            original_brix: 1.,
            final_brix: 1.,
        }
    }

    pub fn original_brix(mut self, brix: f32) -> Self {
        self.original_brix = brix;
        self
    }

    pub fn original_gravity(mut self, gravity: f32) -> Self {
        self.original_brix = Abv::gravity_to_brix(gravity);
        self
    }

    pub fn final_brix(mut self, brix: f32) -> Self {
        self.final_brix = brix;
        self
    }

    pub fn final_gravity(mut self, gravity: f32) -> Self {
        self.final_brix = Self::gravity_to_brix(gravity);
        self
    }

    pub fn abv(&self) -> f32 {
        let og = Self::brix_to_gravity(self.original_brix);
        let fg = Self::brix_to_gravity(self.final_brix);
        return ((76.08 * (og - fg)) / (1.775 - og)) * (fg / 0.794);
    }
}
