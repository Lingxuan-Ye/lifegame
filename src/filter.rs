pub use bit::Bit;
pub use block::Block;
pub use dye::Dye;
pub use emoji::Emoji;

use crate::cell::Cell;
use std::fmt::Display;

mod bit;
mod block;
mod dye;
mod emoji;

/// Visualizes cells.
///
/// Views returned should be visually distinct and of the same size.
pub trait Filter {
    type View: Display;

    fn filter(&self, cell: Cell) -> Self::View;
}
