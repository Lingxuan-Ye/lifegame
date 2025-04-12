use crossterm::style::Stylize;

pub trait StylizeExt: Stylize {
    fn path(self) -> Self::Styled {
        self.blue()
    }

    fn parameter(self) -> Self::Styled {
        self.yellow()
    }
}

impl<T: Stylize> StylizeExt for T {}
