use crate::cell::Cell;
use crate::style::StylizeExt;
use anyhow::{Result, ensure};
use matreex::{Matrix, Shape};
use rand::{Rng, SeedableRng};
use rand_seeder::Seeder;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Random {
    shape: Shape,
    density: Option<f64>,
    seeder: Option<Seeder>,
}

impl Random {
    pub fn new(shape: Shape) -> Result<Self> {
        Ok(Self {
            shape,
            density: None,
            seeder: None,
        })
    }

    pub fn density(mut self, density: f64) -> Result<Self> {
        ensure!(
            (0.0..=1.0).contains(&density),
            "'{}' must be between 0 and 1",
            "density".parameter()
        );
        self.density = Some(density);
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

        let density = self.density.unwrap_or(0.5);

        Matrix::with_initializer(shape, |_| {
            if rng.random_bool(density) {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .map_err(Into::into)
    }
}
