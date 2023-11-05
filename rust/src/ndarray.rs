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
    fn get_flat_index(&self, index: &[usize]) -> usize {
        let flat_index: usize = index
            .iter()
            .zip(self.shape.iter())
            .map(|(&i, &dimsize)| {
                assert!(i < dimsize, "index out of bound.");
                i
            })
            .zip(self.strides.iter())
            .map(|(x, &y)| x * y)
            .sum();
        flat_index
    }
}

impl<T: Clone + Default> Index<&[usize]> for NDArray<T> {
    type Output = T;
    fn index(&self, index: &[usize]) -> &Self::Output {
        &self.data[self.get_flat_index(index)]
    }
}

impl<T: Clone + Default> IndexMut<&[usize]> for NDArray<T> {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let flat_index = self.get_flat_index(index);
        &mut self.data[flat_index]
    }
}
