use std::ops::{Index, IndexMut};

#[derive(PartialEq, Eq)]
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
    pub fn from_slice(slice: &[T], shape: (usize, usize)) -> Self {
        let size = shape.0 * shape.1;
        let mut data = Vec::from(slice);
        data.resize(size, T::default());
        Self { data, shape }
    }
    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }
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
/// use lifegame::{matrix, matrix::{Matrix}};
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
