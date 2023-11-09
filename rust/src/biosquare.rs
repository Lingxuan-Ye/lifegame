use crate::matrix::Matrix;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::Mutex;

pub trait WorldCreator {
    fn create(&self, nrows: usize, ncols: usize) -> Matrix<i8>;
}

pub type Biased = Box<dyn Iterator<Item = String>>;

pub trait LensFilter {
    fn observe(&self, matrix: &Matrix<i8>) -> Biased;
}

pub struct BioSquare {
    current: Matrix<i8>,
    next: Mutex<Matrix<i8>>,
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
    pub fn generate(&mut self) -> &mut Self {
        (0..self.current.size()).into_par_iter().for_each(|i| {
            let (nrows, ncols) = self.current.shape();
            let row = i / ncols;
            let col = i % ncols;
            let mut state = 0;
            let mut next = self.next.lock().unwrap();
            for row_offet in -1..=1 {
                for col_offset in -1..=1 {
                    if row_offet == 0 && col_offset == 0 {
                        state -= 9 * self.current[(row, col)];
                    } else {
                        state += self.current[(
                            row.checked_add_signed(row_offet).unwrap_or(nrows - 1) % nrows,
                            col.checked_add_signed(col_offset).unwrap_or(ncols - 1) % ncols,
                        )]
                    }
                }
            }
            match state {
                3 => next[(row, col)] = 1,
                x if x < 0 && !(x == -6) && !(x == -7) => next[(row, col)] = 0,
                _ => (),
            }
        });
        self.next.lock().unwrap().overwrite(&mut self.current);
        self
    }
    pub fn observe(&self) -> Biased {
        self.lensfilter.observe(&self.current)
    }
    pub fn reset(&mut self) -> &mut Self {
        let (nrows, ncols) = self.current.shape();
        self.current = self.creator.create(nrows, ncols);
        self
    }
}
