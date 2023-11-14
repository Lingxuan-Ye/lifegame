use crate::matrix::Matrix;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub type Generation = Matrix<bool>;
pub type Biased<'a> = Box<dyn Iterator<Item = String> + 'a>;

pub trait WorldCreator {
    fn create(&self, nrows: usize, ncols: usize) -> Generation;
}

pub trait LensFilter {
    fn observe<'a>(&'a self, gen: &'a Generation) -> Biased<'a>;
}

pub struct BioSquare {
    current: Generation,
    next: Mutex<Generation>,
    pub creator: Box<dyn WorldCreator>,
    pub lensfilter: Box<dyn LensFilter>,
}

impl BioSquare {
    pub fn new(
        nrows: usize,
        ncols: usize,
        world_creator: Box<dyn WorldCreator>,
        lensfilter: Box<dyn LensFilter>,
    ) -> Self {
        let current = world_creator.create(nrows, ncols);
        let next = Mutex::new(current.clone());
        Self {
            current,
            next,
            creator: world_creator,
            lensfilter,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        self.current.shape()
    }

    pub fn nrows(&self) -> usize {
        self.current.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.current.ncols()
    }

    pub fn size(&self) -> usize {
        self.current.size()
    }

    pub fn generate(&mut self) -> &mut Self {
        (0..self.current.size()).into_par_iter().for_each(|i| {
            let (nrows, ncols) = self.current.shape();
            let row = i / ncols;
            let col = i % ncols;

            let mut count = 0;
            for row_offet in -1..=1 {
                for col_offset in -1..=1 {
                    if row_offet == 0 && col_offset == 0 {
                        continue;
                    }
                    if self.current[(
                        row.checked_add_signed(row_offet).unwrap_or(nrows - 1) % nrows,
                        col.checked_add_signed(col_offset).unwrap_or(ncols - 1) % ncols,
                    )] {
                        count += 1;
                    }
                }
            }

            let state = self.current[(row, col)];
            let mut next_generation = self.next.lock().unwrap();
            match (state, count) {
                (true, x) if x != 2 && x != 3 => next_generation[(row, col)] = false,
                (false, 3) => next_generation[(row, col)] = true,
                _ => (),
            }
        });
        self.next.lock().unwrap().overwrite(&mut self.current);
        self
    }

    pub fn observe(&self) -> Biased {
        self.lensfilter.observe(&self.current)
    }

    pub fn population_density(&self) -> f64 {
        let sum = Arc::new(AtomicUsize::new(0));
        let gen = &self.current;
        gen.par_for_each_index(|index| {
            if gen[index] {
                sum.fetch_add(1, Ordering::Relaxed);
            };
        });
        sum.load(Ordering::Relaxed) as f64 / gen.size() as f64
    }

    pub fn reset(&mut self) -> &mut Self {
        let (nrows, ncols) = self.current.shape();
        self.current = self.creator.create(nrows, ncols);
        self
    }
}
