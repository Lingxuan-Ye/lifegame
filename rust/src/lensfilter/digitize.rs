use super::{Project, Symbol};
use crate::biosquare::{Biased, Generation, LensFilter};
use crate::term::ToTermString;

pub struct Digitize(Symbol);

impl Digitize {
    pub fn new() -> Self {
        Digitize(Symbol {
            alive: "１".to_tstr().set_bold().to_string(),
            dead: "０".to_tstr().set_bold_dim().to_string(),
        })
    }

    pub fn ascii_compatible() -> Self {
        Digitize(Symbol {
            alive: "1".to_tstr().set_bold().to_string(),
            dead: "0".to_tstr().set_bold_dim().to_string(),
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
                .map(|row| row.to_tstr().set_color("green").unwrap().to_string()),
        )
    }
}
