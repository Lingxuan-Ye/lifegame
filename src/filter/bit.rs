use super::Filter;
use crate::cell::Cell;
use crossterm::style::{StyledContent, Stylize};
use std::sync::LazyLock;

static HALF_WIDTH: LazyLock<Bit> = LazyLock::new(|| Bit {
    dead: '0'.green().bold().dim(),
    alive: '1'.green().bold(),
});
static FULL_WIDTH: LazyLock<Bit> = LazyLock::new(|| Bit {
    dead: '０'.green().bold().dim(),
    alive: '１'.green().bold(),
});

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bit {
    dead: StyledContent<char>,
    alive: StyledContent<char>,
}

impl Bit {
    pub fn half_width() -> &'static Self {
        &HALF_WIDTH
    }

    pub fn full_width() -> &'static Self {
        &FULL_WIDTH
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
