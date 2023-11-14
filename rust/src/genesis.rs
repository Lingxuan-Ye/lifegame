use crate::biosquare::{Generation, WorldCreator};
use crate::matrix::Matrix;
use rand::Rng;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

pub struct DicingGod {
    pub seed: Option<u64>,
    pub initial_density: f64,
}

impl DicingGod {
    pub fn new(seed: Option<u64>, initial_density: f64) -> Self {
        Self {
            seed,
            initial_density,
        }
    }
}

impl WorldCreator for DicingGod {
    fn create(&self, nrows: usize, ncols: usize) -> Generation {
        let mut rng: ChaCha8Rng;

        if let Some(seed) = self.seed {
            rng = ChaCha8Rng::seed_from_u64(seed);
        } else {
            rng = ChaCha8Rng::from_entropy();
        }

        let data: Vec<bool> = (0..nrows * ncols)
            .map(|_| rng.gen_bool(self.initial_density))
            .collect();

        Matrix::from_slice(&data, (nrows, ncols))
    }
}
