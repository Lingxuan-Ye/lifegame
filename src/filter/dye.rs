use super::Filter;
use crate::cell::Cell;
use crossterm::style::{Color, StyledContent, Stylize};
use rand::seq::IteratorRandom;

const PLACEHOLDER: &str = "  ";
const CHOICES: [Color; 7] = [
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::White,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Dye {
    dead: StyledContent<&'static str>,
    alive: StyledContent<&'static str>,
}

impl Dye {
    pub fn new(dead: Option<Color>, alive: Option<Color>) -> Self {
        let mut buffer = [Color::Reset; 2];

        CHOICES
            .into_iter()
            .choose_multiple_fill(&mut rand::rng(), &mut buffer);

        let (dead, alive) = match (dead, alive) {
            (None, None) => (buffer[0], buffer[1]),
            (None, Some(alive)) => {
                if buffer[0] != alive {
                    (buffer[0], alive)
                } else {
                    (buffer[1], alive)
                }
            }
            (Some(dead), None) => {
                if buffer[0] != dead {
                    (dead, buffer[0])
                } else {
                    (dead, buffer[1])
                }
            }
            (Some(dead), Some(alive)) => (dead, alive),
        };

        let dead = PLACEHOLDER.on(dead);
        let alive = PLACEHOLDER.on(alive);

        Self { dead, alive }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let pair = CHOICES.into_iter().choose_multiple(&mut rng, 2);
        let dead = pair[0];
        let alive = pair[1];
        Self::new(dead, alive)
    }
}

impl Default for Dye {
    fn default() -> Self {
        Self::new(Color::Green, Color::White)
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
