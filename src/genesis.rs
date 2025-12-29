use crate::biosquare::Cell;
use anyhow::Result;
use matreex::{Matrix, Shape};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
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

    pub fn random<S>(&self, density: f64, seed: Option<S>) -> Result<Matrix<Cell>>
    where
        S: Hash,
    {
        let density = if (0.0..=1.0).contains(&density) {
            density
        } else {
            0.5
        };
        let mut rng = Seeder::from(seed).into_rng::<ChaCha8Rng>();
        Matrix::with_initializer(self.shape, |_| Cell::from(rng.random_bool(density)))
            .map_err(Into::into)
    }
}
