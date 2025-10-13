use super::Filter;
use crate::biosquare::Cell;
use crossterm::style::Stylize;
use std::sync::LazyLock;

static DEAD: LazyLock<String> = LazyLock::new(|| "0 ".green().bold().dim().to_string());
static ALIVE: LazyLock<String> = LazyLock::new(|| "1 ".green().bold().to_string());

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bit;

impl Filter for Bit {
    fn filter(&self, cell: Cell) -> &str {
        match cell {
            Cell::Dead => &DEAD,
            Cell::Alive => &ALIVE,
        }
    }
}
