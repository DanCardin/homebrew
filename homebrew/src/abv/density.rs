use core::ops::Add;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct SpecificGravity;

#[derive(Debug, Copy, Clone)]
pub struct Brix;

#[derive(Debug, Copy, Clone)]
pub struct Density<Unit>(f32, PhantomData<Unit>);

impl<Unit> Density<Unit> {
    pub fn new(value: f32) -> Self {
        Self(value, PhantomData)
    }

    // pub fn from_brix(value: f32) -> Density<Brix> {
    //     Density(value, PhantomData)
    // }

    // pub fn from_specific_gravity(value: f32) -> Density<SpecificGravity> {
    //     Density(value, PhantomData)
    // }

    pub fn to_f32(&self) -> f32 {
        self.0
    }
}

impl<Unit> Add<Density<Unit>> for Density<Unit> {
    type Output = Density<Unit>;

    fn add(self, other: Density<Unit>) -> Density<Unit> {
        Density(self.0 + other.0, PhantomData)
    }
}

impl Density<Brix> {
    pub fn to_gravity(&self) -> Density<Brix> {
        let brix = self.0;
        let gravity = (brix / (258.6 - ((brix / 258.2) * 227.1))) + 1.;
        Density(gravity, PhantomData)
    }

    pub fn with_wort_correction_factor(&self, factor: f32) -> Self {
        let correction = (factor - 1.) * self.0;
        Self(self.0 - correction, PhantomData)
    }
}

impl Density<SpecificGravity> {
    pub fn to_brix(&self) -> Density<Brix> {
        let gravity = self.0;
        let brix = ((182.4601 * gravity - 775.6821) * gravity + 1262.7794) * gravity - 669.5622;
        Density(brix, PhantomData)
    }
}
