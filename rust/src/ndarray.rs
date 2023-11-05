use std::ops::{Index, IndexMut};

pub struct NDArray<T: Clone + Default> {
    data: Vec<T>,
    shape: Vec<usize>,
    strides: Vec<usize>,
}

impl<T: Clone + Default> NDArray<T> {
    pub fn new(shape: &[usize]) -> Self {
        let mut strides: Vec<usize> = shape
            .iter()
            .rev()
            .scan(1, |state, &x| {
                *state *= x;
                Some(*state)
            })
            .collect();
        let size = strides.pop().unwrap_or_default();
        strides.reverse();
        strides.push(1);

        NDArray {
            data: vec![T::default(); size],
            shape: Vec::from(shape),
            strides,
        }
    }
    pub fn shape(&self) -> &[usize] {
        &self.shape
    }
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }
}
