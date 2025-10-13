use crate::biosquare::Cell;
use anyhow::{Result, ensure};
use matreex::{Matrix, Shape};
use rand::{Rng, SeedableRng};
use rand_seeder::Seeder;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Random {
    density: f64,
    seeder: Option<Seeder>,
}

impl Random {
    pub fn new() -> Self {
        let density = 0.5;
        let seeder = None;
        Self { density, seeder }
    }

    pub fn density(&mut self, density: f64) -> Result<&mut Self> {
        ensure!(
            (0.0..=1.0).contains(&density),
            "value must be between 0 and 1"
        );
        self.density = density;
        Ok(self)
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

        Matrix::with_initializer(shape, |_| rng.random_bool(self.density).into())
            .map_err(Into::into)
    }
}
