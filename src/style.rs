use crossterm::style::Stylize;

pub trait StylizeExt: Stylize {
    fn path(self) -> Self::Styled {
        self.blue()
    }

    fn identifier(self) -> Self::Styled {
        self.yellow()
    }
}

impl<T: Stylize> StylizeExt for T {}
