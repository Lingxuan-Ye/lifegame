use super::Filter;
use crate::cell::Cell;
use crossterm::style::{StyledContent, Stylize};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bit {
    dead: StyledContent<char>,
    alive: StyledContent<char>,
}

impl Bit {
    pub fn half_width() -> Self {
        let dead = '0'.green().bold().dim();
        let alive = '1'.green().bold();
        Self { dead, alive }
    }

    pub fn full_width() -> Self {
        let dead = '０'.green().bold().dim();
        let alive = '１'.green().bold();
        Self { dead, alive }
    }
}

impl Filter for Bit {
    type View = StyledContent<char>;

    fn filter(&self, cell: Cell) -> Self::View {
        match cell {
            Cell::Dead => self.dead,
            Cell::Alive => self.alive,
        }
    }
}
