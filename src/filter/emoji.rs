use super::Filter;
use crate::cell::Cell;
use rand::seq::IteratorRandom;

const DEAD_SYMBOLS: &str = "🤢🥶🥵😡🤬😈👿🤡👻";
const ALIVE_SYMBOLS: &str = "🤣😊🥰😍🤗🤭😋🤤😤";

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Emoji {
    dead: char,
    alive: char,
}

impl Emoji {
    pub  fn random() -> Self {
        let mut rng = rand::rng();
        let Some(dead) = DEAD_SYMBOLS.chars().choose(&mut rng) else {
            unreachable!()
        };
        let Some(alive) = ALIVE_SYMBOLS.chars().choose(&mut rng) else {
            unreachable!()
        };
        Self { dead, alive }
    }
}

impl Filter for Emoji {
    type View = char;

    fn filter(&self, cell: Cell) -> Self::View {
        match cell {
            Cell::Dead => self.dead,
            Cell::Alive => self.alive,
        }
    }
}
