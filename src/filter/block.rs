use super::Filter;
use crate::cell::Cell;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Block;

impl Filter for Block {
    type View = &'static str;

    fn filter(&self, cell: Cell) -> Self::View {
        match cell {
            Cell::Dead => "  ",
            Cell::Alive => "██",
        }
    }
}
