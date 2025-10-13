pub use bit::Bit;
pub use block::Block;
pub use dye::Dye;
pub use emoji::Emoji;
pub use hanzi::Hanzi;

use crate::biosquare::Cell;

mod bit;
mod block;
mod dye;
mod emoji;
mod hanzi;

/// Visual filter for [`Cell`].
pub trait Filter {
    /// Returns the filtered representation of the given cell.
    ///
    /// The returned values should be visually distinct from each other and of
    /// the same length.
    fn filter(&self, cell: Cell) -> &str;
}

impl Filter for Box<dyn Filter> {
    fn filter(&self, cell: Cell) -> &str {
        self.as_ref().filter(cell)
    }
}
