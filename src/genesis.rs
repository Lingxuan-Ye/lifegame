use crate::cell::Cell;
use crate::style::StylizeExt;
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
        Self {
            density: 0.5,
            seeder: None,
        }
    }

    pub fn density(mut self, density: f64) -> Result<Self> {
        ensure!(
            (0.0..=1.0).contains(&density),
            "'{}' must be between 0 and 1",
            "density".parameter()
        );
        self.density = density;
        Ok(self)
    }

    pub fn seed<S>(mut self, seed: S) -> Self
    where
        S: Hash,
    {
        self.seeder = Some(Seeder::from(seed));
        self
    }

    pub fn generate<R>(self, shape: Shape) -> Result<Matrix<Cell>>
    where
        R: Rng + SeedableRng,
    {
        let mut rng: R = match self.seeder {
            None => SeedableRng::try_from_os_rng()?,
            Some(mut seeder) => seeder.into_rng(),
        };

        Matrix::with_initializer(shape, |_| {
            if rng.random_bool(self.density) {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .map_err(Into::into)
    }
}
