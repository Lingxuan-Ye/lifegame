use super::Filter;
use crate::biosquare::Cell;
use crossterm::style::{StyledContent, Stylize};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bit;

impl Filter for Bit {
    type View = StyledContent<&'static str>;

    fn filter(&self, cell: Cell) -> Self::View {
        match cell {
            Cell::Dead => "0 ".green().bold().dim(),
            Cell::Alive => "1 ".green().bold(),
        }
    }
}
