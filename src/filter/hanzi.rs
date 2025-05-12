use super::Filter;
use crate::cell::Cell;
use crossterm::style::{StyledContent, Stylize};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Hanzi;

impl Filter for Hanzi {
    type View = StyledContent<char>;

    fn filter(&self, cell: Cell) -> Self::View {
        match cell {
            Cell::Dead => '死'.bold().dim(),
            Cell::Alive => '生'.bold(),
        }
    }
}
