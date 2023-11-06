use std::ops::{Index, IndexMut};

pub struct Matrix<T: Clone + Default> {
    data: Vec<T>,
    shape: (usize, usize),
}

impl<T: Clone + Default> Matrix<T> {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        let size: usize = nrows * ncols;
        Matrix {
            data: vec![T::default(); size],
            shape: (nrows, ncols),
        }
    }
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }
}

impl<T: Clone + Default> Matrix<T> {
    fn flat_index(&self, index: (usize, usize)) -> usize {
        assert!(index.0 < self.shape.0, "index 0 out of bound.");
        assert!(index.1 < self.shape.1, "index 1 out of bound.");
        index.0 * self.shape.1 + index.1
    }
}

impl<T: Clone + Default> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.flat_index(index)]
    }
}

impl<T: Clone + Default> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let flattened = self.flat_index(index);
        &mut self.data[flattened]
    }
}
