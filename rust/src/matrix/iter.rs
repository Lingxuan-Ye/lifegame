use super::Matrix;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
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

type Row<'a, T> = Take<Skip<Iter<'a, T>>>;

impl<'a, T: Clone + Default> Iterator for RowIter<'a, T> {
    type Item = Row<'a, T>;
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

type Col<'a, T> = StepBy<Skip<Iter<'a, T>>>;

impl<'a, T: Clone + Default> Iterator for ColIter<'a, T> {
    type Item = Col<'a, T>;
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
    pub fn iterrows(&self) -> impl Iterator<Item = Row<T>> {
        RowIter::new(self)
    }
    pub fn itercols(&self) -> impl Iterator<Item = Col<T>> {
        ColIter::new(self)
    }
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        (0..self.size()).for_each(|i| f(i / self.shape.1, i % self.shape.1))
    }
    pub fn map<U, F>(&self, f: F) -> Matrix<U>
    where
        U: Clone + Default,
        F: FnMut(&T) -> U,
    {
        Matrix {
            data: self.data.iter().map(f).collect(),
            shape: self.shape,
        }
    }
}

impl<T: Clone + Default + Sync> Matrix<T> {
    pub fn par_for_each<F>(&self, f: F)
    where
        F: Fn(usize, usize) + Sync,
    {
        (0..self.size())
            .into_par_iter()
            .for_each(|i| f(i / self.shape.1, i % self.shape.1))
    }
    pub fn par_map<U, F>(&self, f: F) -> Matrix<U>
    where
        U: Clone + Default + Send,
        F: Fn(&T) -> U + Send + Sync,
    {
        Matrix {
            data: self.data.par_iter().map(f).collect(),
            shape: self.shape,
        }
    }
}
