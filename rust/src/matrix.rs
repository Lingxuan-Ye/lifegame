use std::ops::{Index, IndexMut};

use crate::ndarray::NDArray;

pub struct Matrix<T: Clone + Default>(NDArray<T>);

impl<T: Clone + Default> Matrix<T> {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Matrix(NDArray::new(&[nrows, ncols]))
    }
    pub fn shape(&self) -> &[usize] {
        self.0.shape()
    }
    pub fn nrows(&self) -> usize {
        self.shape()[0]
    }
    pub fn ncols(&self) -> usize {
        self.shape()[1]
    }
}

impl<T: Clone + Default> Index<&[usize]> for Matrix<T> {
    type Output = T;
    fn index(&self, index: &[usize]) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Clone + Default> IndexMut<&[usize]> for Matrix<T> {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        &mut self.0[index]
    }
}
