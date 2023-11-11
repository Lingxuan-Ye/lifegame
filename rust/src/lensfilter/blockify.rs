use super::{Project, Symbol};
use crate::biosquare::{Biased, Generation, LensFilter};

pub struct Blockify(Symbol);

impl Blockify {
    pub fn new() -> Self {
        Blockify(Symbol {
            alive: "██".to_string(),
            dead: "  ".to_string(),
        })
    }
}

impl Project for Blockify {
    fn sym_alive(&self) -> &str {
        &self.0.alive
    }
    fn sym_dead(&self) -> &str {
        &self.0.dead
    }
}

impl LensFilter for Blockify {
    fn observe<'a>(&'a self, gen: &'a Generation) -> Biased<'a> {
        self.project(gen)
    }
}
