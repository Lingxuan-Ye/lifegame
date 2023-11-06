use super::Matrix;
use std::ops::{Index, IndexMut};

impl<T: Clone + Default> Matrix<T> {
    fn flat_index(&self, index: (usize, usize)) -> usize {
        assert!(index.0 < self.shape.0, "index 0 out of bounds.");
        assert!(index.1 < self.shape.1, "index 1 out of bounds.");
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
