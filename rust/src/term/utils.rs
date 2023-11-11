use super::{TermString, ESCSEQ};

#[inline]
pub fn erase_screen() {
    println!("{}", &ESCSEQ["erase"]["screen"]);
}

#[inline]
pub fn reset_cursor() {
    print!("{}", &ESCSEQ["reset"]["cursor"]);
}

#[inline]
fn get_tstr(cat: &str, key: &str) -> TermString {
    TermString::from_escseq(&ESCSEQ[cat][key])
}

#[inline]
pub fn set_bold(value: &str) -> TermString {
    get_tstr("style", "bold") + value + get_tstr("reset", "bold/dim")
}

#[inline]
pub fn set_dim(value: &str) -> TermString {
    get_tstr("style", "dim") + value + get_tstr("reset", "bold/dim")
}

#[inline]
pub fn set_bold_dim(value: &str) -> TermString {
    get_tstr("style", "bold") + get_tstr("style", "dim") + value + get_tstr("reset", "bold/dim")
}

#[inline]
pub fn set_color(value: &str, color: &str) -> TermString {
    get_tstr("foreground", color) + value + get_tstr("reset", "foreground")
}

#[inline]
pub fn set_color_bg(value: &str, color: &str) -> TermString {
    get_tstr("background", color) + value + get_tstr("reset", "background")
}

#[inline]
pub fn set_bold_t(value: TermString) -> TermString {
    get_tstr("style", "bold") + value + get_tstr("reset", "bold/dim")
}

#[inline]
pub fn set_dim_t(value: TermString) -> TermString {
    get_tstr("style", "dim") + value + get_tstr("reset", "bold/dim")
}

#[inline]
pub fn set_bold_dim_t(value: TermString) -> TermString {
    get_tstr("style", "bold") + get_tstr("style", "dim") + value + get_tstr("reset", "bold/dim")
}

#[inline]
pub fn set_color_t(value: TermString, color: &str) -> TermString {
    get_tstr("foreground", color) + value + get_tstr("reset", "foreground")
}

#[inline]
pub fn set_color_bg_t(value: TermString, color: &str) -> TermString {
    get_tstr("background", color) + value + get_tstr("reset", "background")
}
