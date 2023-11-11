#[macro_export]
macro_rules! erase_screen {
    () => {
        std::println!($crate::term::ESCSEQ["erase"]["screen"]);
    };
}

#[macro_export]
macro_rules! reset_cursor {
    () => {
        std::print!($crate::term::ESCSEQ["reset"]["cursor"]);
    };
}

#[macro_export]
macro_rules! get_tstr {
    ($cat:expr, $key:expr) => {
        $crate::term::TermString::from_escseq($crate::term::ESCSEQ[$x][$y])
    };
}

#[macro_export]
macro_rules! set_bold {
    ($x:expr) => {
        $crate::get_tstr!("style", "bold") + $x + $crate::get_tstr!("reset", "bold/dim")
    };
}

#[macro_export]
macro_rules! set_dim {
    ($x:expr) => {
        $crate::get_tstr!("style", "dim") + $x + $crate::get_tstr!("reset", "bold/dim")
    };
}

#[macro_export]
macro_rules! set_bold_dim {
    ($x:expr) => {
        $crate::get_tstr!("style", "bold")
            + $crate::get_tstr!("style", "dim")
            + $x
            + $crate::get_tstr!("reset", "bold/dim")
    };
}

#[macro_export]
macro_rules! set_fg {
    ($x:expr, $color:expr) => {
        $crate::get_tstr!("foreground", $color) + $x + $crate::get_tstr!("reset", "foreground")
    };
}

#[macro_export]
macro_rules! set_bg {
    ($x:expr, $color:expr) => {
        $crate::get_tstr!("background", $color) + $x + $crate::get_tstr!("reset", "background")
    };
}
