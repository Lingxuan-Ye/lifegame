pub mod blockify;
pub mod digitize;
pub mod dye;
pub mod emojify;

pub use blockify::Blockify;
pub use digitize::Digitize;
pub use dye::Dye;
pub use emojify::Emojify;

use crate::biosquare::{Biased, Generation};
use std::cmp::max;

trait Project {
    fn sym_alive(&self) -> &str;

    fn sym_dead(&self) -> &str;

    fn project<'a>(&'a self, gen: &'a Generation) -> Biased<'a> {
        let line_len = max(self.sym_alive().len(), self.sym_dead().len()) * gen.ncols();
        return Box::new(gen.iterrows().map(move |row| {
            let mut line = String::with_capacity(line_len);
            for &status in row {
                if status {
                    line.push_str(&self.sym_alive())
                } else {
                    line.push_str(&self.sym_dead())
                }
            }
            line
        }));
    }
}

struct Symbol {
    alive: String,
    dead: String,
}
