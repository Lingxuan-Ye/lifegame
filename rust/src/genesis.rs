use crate::{biosquare::WorldCreator, matrix::Matrix};
use rand::Rng;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

pub struct DicingGod {
    pub seed: u64,
    pub population_density: f64,
}

impl DicingGod {
    pub fn new(seed: u64, population_density: f64) -> Self {
        Self {
            seed,
            population_density,
        }
    }
}

impl WorldCreator for DicingGod {
    fn create(&self, nrows: usize, ncols: usize) -> Matrix<i8> {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        let data: Vec<i8> = (0..nrows * ncols)
            .map(|_| rng.gen_bool(self.population_density) as i8)
            .collect();
        Matrix::from_slice(&data, (nrows, ncols))
    }
}
