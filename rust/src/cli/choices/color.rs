use super::{StyleChoice, DEFAULT_TAG};
use crate::term::ToTermString;
use clap::ValueEnum;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value().unwrap().get_name().fmt(f)
    }
}

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, true) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

impl StyleChoice for Color {
    fn style(default: Self) -> String {
        let mut choices: Vec<String> = Vec::with_capacity(8);
        for variant in Color::value_variants() {
            let variant_s = variant.to_string();
            let mut variant_t = format!("  {variant_s:<9}")
                .to_tstr()
                .set_bold()
                .set_color_bg(&variant_s)
                .unwrap();
            if *variant == Color::White {
                variant_t = variant_t.set_color("black").unwrap();
            }
            if *variant == default {
                variant_t += &DEFAULT_TAG as &str;
            }
            choices.push(format!("  - {variant_t}"));
        }
        choices.join("\n")
    }
}
