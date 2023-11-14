mod display;
mod ops;

use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ESCSEQ: Lazy<HashMap<String, HashMap<String, String>>> =
    Lazy::new(|| serde_json::from_str(include_str!("../resources/escseq.json")).unwrap());

#[inline]
pub fn erase_screen() {
    print!("{}", &ESCSEQ["erase"]["screen"]);
}

#[inline]
pub fn reset_cursor() {
    print!("{}", &ESCSEQ["reset"]["cursor"]);
}

pub struct TermString {
    data: String,
    esc_len: usize,
}

impl TermString {
    pub fn wrap<T: ToString>(seq: &T) -> Self {
        TermString {
            data: seq.to_string(),
            esc_len: 0,
        }
    }

    pub fn from_escseq(cat: &str, key: &str) -> Option<Self> {
        let seq = ESCSEQ.get(cat)?.get(key)?;
        Some(TermString {
            data: seq.clone(),
            esc_len: seq.len(),
        })
    }

    pub fn len(&self) -> usize {
        self.data.len() - self.esc_len
    }

    pub fn center(mut self, mut width: usize) -> Self {
        width += self.esc_len;
        self.data = format!("{:^width$}", self.data);
        self
    }

    pub fn ljust(mut self, mut width: usize) -> Self {
        width += self.esc_len;
        self.data = format!("{:<width$}", self.data);
        self
    }

    pub fn rjust(mut self, mut width: usize) -> Self {
        width += self.esc_len;
        self.data = format!("{:>width$}", self.data);
        self
    }

    pub fn set_bold(self) -> Self {
        Self::from_escseq("style", "bold").unwrap()
            + self
            + Self::from_escseq("reset", "bold/dim").unwrap()
    }

    pub fn set_dim(self) -> Self {
        Self::from_escseq("style", "dim").unwrap()
            + self
            + Self::from_escseq("reset", "bold/dim").unwrap()
    }

    pub fn set_bold_dim(self) -> Self {
        Self::from_escseq("style", "bold").unwrap()
            + Self::from_escseq("style", "dim").unwrap()
            + self
            + Self::from_escseq("reset", "bold/dim").unwrap()
    }

    pub fn set_italic(self) -> Self {
        Self::from_escseq("style", "italic").unwrap()
            + self
            + Self::from_escseq("reset", "italic").unwrap()
    }

    pub fn set_underline(self) -> Self {
        Self::from_escseq("style", "underline").unwrap()
            + self
            + Self::from_escseq("reset", "underline").unwrap()
    }

    pub fn set_strikethrough(self) -> Self {
        Self::from_escseq("style", "strikethrough").unwrap()
            + self
            + Self::from_escseq("reset", "strikethrough").unwrap()
    }

    pub fn set_color(self, color: &str) -> Result<Self, &'static str> {
        if let Some(color_seq) = Self::from_escseq("foreground", color) {
            Ok(color_seq + self + Self::from_escseq("reset", "foreground").unwrap())
        } else {
            Err("invalid color")
        }
    }

    pub fn set_color_bg(self, color: &str) -> Result<Self, &'static str> {
        if let Some(color_seq) = Self::from_escseq("background", color) {
            Ok(color_seq + self + Self::from_escseq("reset", "background").unwrap())
        } else {
            Err("invalid color")
        }
    }
}

pub trait ToTermString {
    fn to_tstr(&self) -> TermString;
}

impl<T: ToString> ToTermString for T {
    fn to_tstr(&self) -> TermString {
        TermString::wrap(self)
    }
}
