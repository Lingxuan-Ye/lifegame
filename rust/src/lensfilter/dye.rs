use super::{Project, Symbol};
use crate::biosquare::{Biased, Generation, LensFilter};
use crate::term::ESCSEQ;
use rand::seq::{IteratorRandom, SliceRandom};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

const FSPACE: &str = "　";

pub struct Dye(Symbol);

impl Dye {
    pub fn new(color_alive: &str, color_dead: &str) -> Self {
        Dye(Symbol {
            alive: ESCSEQ["background"][color_alive].clone() + FSPACE,
            dead: ESCSEQ["background"][color_dead].clone() + FSPACE,
        })
    }
    pub fn random() -> Self {
        let mut rng = ChaCha8Rng::from_entropy();
        let mut pair = ESCSEQ["background"].keys().choose_multiple(&mut rng, 2);
        pair.shuffle(&mut rng);
        Self::new(pair[0], pair[1])
    }
}

impl Project for Dye {
    fn sym_alive(&self) -> &str {
        &self.0.alive
    }
    fn sym_dead(&self) -> &str {
        &self.0.dead
    }
}

impl LensFilter for Dye {
    fn observe<'a>(&'a self, gen: &'a Generation) -> Biased<'a> {
        Box::new(
            self.project(gen)
                .map(|row| row + ESCSEQ["reset"]["all"].as_str()),
        )
    }
}
