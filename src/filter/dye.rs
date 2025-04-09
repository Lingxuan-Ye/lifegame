use super::Filter;
use crate::cell::Cell;
use crossterm::style::{Color, StyledContent, Stylize};
use rand::seq::IteratorRandom;
use std::sync::LazyLock;

const PLACEHOLDER: &str = "  ";
const COLORS: [Color; 7] = [
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::White,
];
static PRESET: LazyLock<Dye> = LazyLock::new(|| Dye::new(Color::Green, Color::White));

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Dye {
    dead: StyledContent<&'static str>,
    alive: StyledContent<&'static str>,
}

impl Dye {
    pub fn new(dead: Color, alive: Color) -> Self {
        let dead = PLACEHOLDER.on(dead);
        let alive = PLACEHOLDER.on(alive);
        Dye { dead, alive }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let pair = COLORS.into_iter().choose_multiple(&mut rng, 2);
        let dead = pair[0];
        let alive = pair[1];
        Dye::new(dead, alive)
    }

    pub fn preset() -> &'static Self {
        &PRESET
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
