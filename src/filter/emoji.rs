use super::Filter;
use crate::biosquare::Cell;
use rand::Rng;

const DEAD_SYMBOLS: &str = "🤢🥶🥵😡🤬😈👿🤡👻";
const ALIVE_SYMBOLS: &str = "🤣😊🥰😍🤗🤭😋🤤😤";

#[derive(Debug, PartialEq, Eq)]
pub struct Emoji {
    dead: &'static str,
    alive: &'static str,
}

impl Emoji {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let dead = Self::random_char(DEAD_SYMBOLS, &mut rng);
        let alive = Self::random_char(ALIVE_SYMBOLS, &mut rng);
        Self { dead, alive }
    }

    fn random_char<'a, R>(slice: &'a str, rng: &mut R) -> &'a str
    where
        R: Rng,
    {
        let len = slice.chars().count();
        let index = rng.random_range(..len);
        let mut iter = slice.char_indices().skip(index).map(|(index, _)| index);
        let lower = iter.next().unwrap();
        let upper = iter.next().unwrap_or(slice.len());
        &slice[lower..upper]
    }
}

impl Filter for Emoji {
    fn filter(&self, cell: Cell) -> &str {
        match cell {
            Cell::Dead => self.dead,
            Cell::Alive => self.alive,
        }
    }
}
