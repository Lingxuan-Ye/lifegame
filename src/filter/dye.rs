use super::Filter;
use crate::cell::Cell;
use crossterm::style::{Color, StyledContent, Stylize};

const PLACEHOLDER: &str = "  ";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Dye {
    dead: StyledContent<&'static str>,
    alive: StyledContent<&'static str>,
}

impl Dye {
    pub fn new(dead: Color, alive: Color) -> Self {
        let dead = PLACEHOLDER.on(dead);
        let alive = PLACEHOLDER.on(alive);
        Self { dead, alive }
    }
}

impl Filter for Dye {
    type View = StyledContent<&'static str>;

    fn filter(&self, cell: Cell) -> Self::View {
        match cell {
            Cell::Dead => self.dead,
            Cell::Alive => self.alive,
        }
    }
}
