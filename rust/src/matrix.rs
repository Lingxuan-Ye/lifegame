use rayon::prelude::*;
use std::sync::{Arc, Mutex};

mod display;
mod index;

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<T: Clone + Default> {
    data: Vec<T>,
    shape: (usize, usize),
}

impl<T: Clone + Default> Matrix<T> {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        let size: usize = nrows * ncols;
        Self {
            data: vec![T::default(); size],
            shape: (nrows, ncols),
        }
    }
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }
    pub fn nrows(&self) -> usize {
        self.shape.0
    }
    pub fn ncols(&self) -> usize {
        self.shape.1
    }
    pub fn size(&self) -> usize {
        self.shape.0 * self.shape.1
    }
    pub fn can_hold<U: Clone + Default>(&self, other: &Matrix<U>) -> bool {
        self.nrows() >= other.nrows() && self.ncols() >= other.ncols()
    }
    pub fn from_slice(slice: &[T], shape: (usize, usize)) -> Self {
        let size = shape.0 * shape.1;
        let mut data = Vec::from(slice);
        data.resize(size, T::default());
        Self { data, shape }
    }
}

/// Creates a [`Matrix`] with 2-dimensional array literal.
///
///
/// # Panics
///
/// `matrix!` panics if row lengths of the given array are inconsistent.
///
/// # Examples
///
/// ```
/// use lifegame::{matrix, matrix::Matrix};
///
/// matrix![[0, 1, 2, 3, 4], [5, 6, 7, 8, 9],];
/// let foo: Matrix<i32> = matrix![];
/// let bar: Matrix<u8> = matrix![[0, 1, 2], [3, 4, 5]];
/// assert!(bar.shape() == (2, 3));
/// ```
#[macro_export]
macro_rules! matrix {
    [ $( [ $($x:expr),* $(,)? ] ),* $(,)? ] => {
        {
        #[allow(unused_mut)]
        let mut data = std::vec::Vec::new();
        #[allow(unused_mut)]
        let mut shape: (usize, usize) = (0, 0);
        $(
            shape.0 += 1;
            if shape.1 == 0 {
                $(
                    shape.1 += 1;
                    data.push($x);
                )*
            } else {
                #[allow(unused_mut)]
                let mut count: usize = 0;
                $(
                    count += 1;
                    data.push($x);
                )*
                assert!(count == shape.1, "inconsistent row lengths detected.")
            }
        )*
        $crate::matrix::Matrix::from_slice(&data, shape)
    }};
}
