use super::Filter;
use crate::biosquare::Cell;
use crossterm::style::Stylize;
use std::sync::LazyLock;

static DEAD: LazyLock<String> = LazyLock::new(|| '死'.bold().dim().to_string());
static ALIVE: LazyLock<String> = LazyLock::new(|| '生'.bold().to_string());

#[derive(Debug, PartialEq, Eq)]
pub struct Hanzi;

impl Filter for Hanzi {
    fn filter(&self, cell: Cell) -> &str {
        match cell {
            Cell::Dead => &DEAD,
            Cell::Alive => &ALIVE,
        }
    }
}
