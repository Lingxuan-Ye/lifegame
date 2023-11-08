use super::Matrix;
use std::{
    iter::{Skip, StepBy, Take},
    slice::Iter,
};

pub struct RowIter<'a, T: Clone + Default> {
    matrix: &'a Matrix<T>,
    count: usize,
}

impl<'a, T: Clone + Default> RowIter<'a, T> {
    fn new(matrix: &'a Matrix<T>) -> Self {
        Self { matrix, count: 0 }
    }
}

impl<'a, T: Clone + Default> Iterator for RowIter<'a, T> {
    type Item = Take<Skip<Iter<'a, T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.matrix.nrows() {
            return None;
        }
        self.count += 1;
        let iter = self
            .matrix
            .data
            .iter()
            .skip(self.count * self.matrix.ncols())
            .take(self.matrix.ncols());
        Some(iter)
    }
}

pub struct ColIter<'a, T: Clone + Default> {
    matrix: &'a Matrix<T>,
    count: usize,
}

impl<'a, T: Clone + Default> ColIter<'a, T> {
    fn new(matrix: &'a Matrix<T>) -> Self {
        Self { matrix, count: 0 }
    }
}

impl<'a, T: Clone + Default> Iterator for ColIter<'a, T> {
    type Item = StepBy<Skip<Iter<'a, T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.matrix.ncols() {
            return None;
        }
        self.count += 1;
        let iter = self
            .matrix
            .data
            .iter()
            .skip(self.count)
            .step_by(self.matrix.nrows());
        Some(iter)
    }
}

impl<T: Clone + Default> Matrix<T> {
    pub fn iterrows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        RowIter::new(self)
    }
    pub fn itercols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        ColIter::new(self)
    }
}
