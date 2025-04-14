use crate::cell::Cell;
use matreex::{Matrix, WrappingIndex};

#[derive(Clone, Debug)]
pub struct BioSquare {
    generation: usize,
    population: usize,
    current: Matrix<Cell>,
    next: Matrix<Cell>,
}

impl BioSquare {
    pub fn new(genesis: Matrix<Cell>) -> Self {
        let generation = 0;
        let population = genesis
            .iter_elements()
            .filter(|cell| cell.is_alive())
            .count();
        let current = genesis;
        let next = current.clone();

        Self {
            generation,
            population,
            current,
            next,
        }
    }

    pub fn observe(&self) -> &Matrix<Cell> {
        &self.current
    }

    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn population(&self) -> usize {
        self.population
    }

    pub fn density(&self) -> f64 {
        self.population as f64 / self.observe().size() as f64
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

        self.generation += 1;

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
                        self.population += 1;
                    }
                }
                Cell::Alive => {
                    if !(2..=3).contains(&neighbors) {
                        cell.die();
                        self.population -= 1;
                    }
                }
            }
        }

        self.current.overwrite(&self.next);

        self
    }
}

impl Iterator for BioSquare {
    type Item = Matrix<Cell>;

    fn next(&mut self) -> Option<Self::Item> {
        let matrix = self.observe().clone();
        self.evolve();
        Some(matrix)
    }
}
