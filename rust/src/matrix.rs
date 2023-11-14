mod display;
mod index;
mod iter;

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<T: Clone + Default = i32> {
    data: Vec<T>,
    shape: (usize, usize),
}

impl<T: Clone + Default> Matrix<T> {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        let size = Self::cal_size(nrows, ncols);
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

    fn cal_size(nrows: usize, ncols: usize) -> usize {
        nrows.checked_mul(ncols).expect("size overflows")
    }

    pub fn size(&self) -> usize {
        Self::cal_size(self.shape.0, self.shape.1)
    }

    pub fn can_hold<U: Clone + Default>(&self, other: &Matrix<U>) -> bool {
        self.shape.0 >= other.shape.0 && self.shape.1 >= other.shape.1
    }

    pub fn overwrite(&self, other: &mut Matrix<T>) {
        assert!(other.can_hold(self));
        for (index, value) in self.data.iter().enumerate() {
            other.data[index] = value.clone()
        }
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
///
/// let foo: Matrix<i32> = matrix![];
/// assert!(foo.shape() == (0, 0));
/// assert!(foo.size() == 0);
///
/// let bar: Matrix<u8> = matrix![[0, 1, 2], [3, 4, 5]];
/// assert!(bar.shape() == (2, 3));
/// assert!(bar.size() == 6);
/// ```
#[macro_export]
macro_rules! matrix {
    [ $( [ $($x:expr),* $(,)? ] ),* $(,)? ] => {{
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
                assert!(count == shape.1, "inconsistent row lengths detected")
            }
        )*
        $crate::matrix::Matrix::from_slice(&data, shape)
    }};
}
