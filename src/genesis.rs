use crate::biosquare::Cell;
use anyhow::Result;
use matreex::{Matrix, Shape};
use rand::RngExt;
use rand::rngs::ChaCha8Rng;
use rand_seeder::Seeder;
use std::hash::Hash;

#[derive(Debug)]
pub struct Genesis {
    shape: Shape,
}

impl Genesis {
    pub fn new(shape: Shape) -> Self {
        Self { shape }
    }

    pub fn random<S>(&self, density: Density, seed: Option<S>) -> Result<Matrix<Cell>>
    where
        S: Hash,
    {
        let mut rng: ChaCha8Rng = match seed {
            None => rand::make_rng(),
            Some(seed) => Seeder::from(seed).into_rng(),
        };
        Matrix::from_fn(self.shape, |_| Cell::from(rng.random_bool(density.0))).map_err(Into::into)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Density(f64);

impl Density {
    pub fn new(value: f64) -> Option<Self> {
        if (0.0..=1.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}
