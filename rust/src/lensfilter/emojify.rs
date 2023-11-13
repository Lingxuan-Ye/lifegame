use super::{Project, Symbol};
use crate::biosquare::{Biased, Generation, LensFilter};
use rand::seq::IteratorRandom;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

pub struct Emojify(Symbol);

impl Emojify {
    pub fn new(emoji_alive: char, emoji_dead: char) -> Self {
        Emojify(Symbol {
            alive: String::from(emoji_alive),
            dead: String::from(emoji_dead),
        })
    }
    pub fn random() -> Self {
        let mut rng = ChaCha8Rng::from_entropy();
        let emoji_alive = "😆🤣😊🥰😍🤗🤭😋🤤😤".chars().choose(&mut rng).unwrap();
        let emoji_dead = "🤢🥶🥵😡🤬😈👿🤡👻".chars().choose(&mut rng).unwrap();
        Self::new(emoji_alive, emoji_dead)
    }
}

impl Project for Emojify {
    fn sym_alive(&self) -> &str {
        &self.0.alive
    }
    fn sym_dead(&self) -> &str {
        &self.0.dead
    }
}

impl LensFilter for Emojify {
    fn observe<'a>(&'a self, gen: &'a Generation) -> Biased<'a> {
        self.project(gen)
    }
}
