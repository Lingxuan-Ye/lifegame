use crossterm::style::Stylize;

pub trait StylizeExt: Stylize {
    fn path(self) -> Self::Styled {
        self.blue()
    }

    fn parameter(self) -> Self::Styled {
        self.yellow()
    }

    fn key(self) -> Self::Styled {
        self.bold()
    }
}

impl<T: Stylize> StylizeExt for T {}
