use crate::biosquare::Cell;
use crate::bounded::Bounded;
use anyhow::Result;
use matreex::{Matrix, Shape};
use rand::{Rng, SeedableRng};
use rand_seeder::Seeder;
use std::hash::Hash;
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct Random {
    density: Density,
    seeder: Option<Seeder>,
}

impl Random {
    pub fn new() -> Self {
        let density = Density::default();
        let seeder = None;
        Self { density, seeder }
    }

    pub fn density(&mut self, density: Density) -> &mut Self {
        self.density = density;
        self
    }

    pub fn seed<S>(&mut self, seed: Option<S>) -> &mut Self
    where
        S: Hash,
    {
        self.seeder = seed.map(Seeder::from);
        self
    }

    pub fn generate<R>(&mut self, shape: Shape) -> Result<Matrix<Cell>>
    where
        R: Rng + SeedableRng,
    {
        let mut rng: R = match &mut self.seeder {
            None => SeedableRng::try_from_os_rng()?,
            Some(seeder) => seeder.into_rng(),
        };

        Matrix::with_initializer(shape, |_| {
            let density = *self.density.get();
            let state = rng.random_bool(density);
            Cell::from(state)
        })
        .map_err(Into::into)
    }
}

#[derive(Clone, Debug)]
pub struct Density(f64);

impl Bounded<f64, RangeInclusive<f64>> for Density {
    const RANGE: RangeInclusive<f64> = 0.0..=1.0;

    fn new_or_default(value: f64) -> Self {
        if Self::RANGE.contains(&value) {
            Self(value)
        } else {
            Self::default()
        }
    }

    fn get(&self) -> &f64 {
        &self.0
    }
}

impl Default for Density {
    fn default() -> Self {
        Self(0.5)
    }
}
