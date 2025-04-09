use crate::cell::Cell;
use matreex::{Matrix, WrappingIndex};
use std::mem::swap;

#[derive(Clone, Debug)]
pub struct BioSquare {
    current: Matrix<Cell>,
    next: Matrix<Cell>,
}

impl BioSquare {
    pub fn new(genesis: Matrix<Cell>) -> Self {
        let current = genesis;
        let next = current.clone();
        Self { current, next }
    }

    pub fn observe(&self) -> &Matrix<Cell> {
        &self.current
    }

    pub fn density(&self) -> f64 {
        let generation = self.observe();
        let total_cells = generation.size();
        let alive_cells = generation
            .iter_elements()
            .filter(|cell| cell.is_alive())
            .count();
        alive_cells as f64 / total_cells as f64
    }

    pub fn evolve(&mut self) -> &mut Self {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (index, cell) in self.next.iter_elements_mut_with_index() {
            let neighbors = OFFSETS
                .into_iter()
                .filter(|offset| {
                    // For a cell matrix, the numbers of rows and columns are
                    // guaranteed to be within `0..=isize::MAX as usize` due
                    // to allocation limits. Therefore, the following cast is
                    // always safe.
                    let index = WrappingIndex::new(
                        index.row as isize + offset.0,
                        index.col as isize + offset.1,
                    );
                    self.current[index].is_alive()
                })
                .count();

            match cell {
                Cell::Dead => {
                    if neighbors == 3 {
                        cell.revive();
                    }
                }
                Cell::Alive => {
                    if !(2..=3).contains(&neighbors) {
                        cell.die();
                    }
                }
            }
        }

        swap(&mut self.current, &mut self.next);

        self
    }
}
