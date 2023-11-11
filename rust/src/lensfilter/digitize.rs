use super::{Project, Symbol};
use crate::biosquare::{Biased, Generation, LensFilter};
use crate::term::utils::{set_bold, set_bold_dim, set_color};

pub struct Digitize(Symbol);

impl Digitize {
    pub fn new() -> Self {
        Digitize(Symbol {
            alive: set_bold("１").to_string(),
            dead: set_bold_dim("０").to_string(),
        })
    }
}

impl Project for Digitize {
    fn sym_alive(&self) -> &str {
        &self.0.alive
    }
    fn sym_dead(&self) -> &str {
        &self.0.dead
    }
}

impl LensFilter for Digitize {
    fn observe<'a>(&'a self, gen: &'a Generation) -> Biased<'a> {
        Box::new(
            self.project(gen)
                .map(|row| set_color(&row, "green").to_string()),
        )
    }
}
