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

    fn unit(self) -> Self::Styled {
        self.dim()
    }
}

impl<T: Stylize> StylizeExt for T {}
