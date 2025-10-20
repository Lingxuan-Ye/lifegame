use super::Filter;
use crate::biosquare::Cell;
use crossterm::style::{Color, Stylize};

const PLACEHOLDER: &str = "  ";

#[derive(Debug, PartialEq, Eq)]
pub struct Dye {
    dead: String,
    alive: String,
}

impl Dye {
    pub fn new(dead: Color, alive: Color) -> Self {
        let dead = PLACEHOLDER.on(dead).to_string();
        let alive = PLACEHOLDER.on(alive).to_string();
        Self { dead, alive }
    }
}

impl Filter for Dye {
    fn filter(&self, cell: Cell) -> &str {
        match cell {
            Cell::Dead => &self.dead,
            Cell::Alive => &self.alive,
        }
    }
}
