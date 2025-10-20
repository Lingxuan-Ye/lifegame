use super::Filter;
use crate::biosquare::Cell;

#[derive(Debug, PartialEq, Eq)]
pub struct Block;

impl Filter for Block {
    fn filter(&self, cell: Cell) -> &str {
        match cell {
            Cell::Dead => "  ",
            Cell::Alive => "██",
        }
    }
}
