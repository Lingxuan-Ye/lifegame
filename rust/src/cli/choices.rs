use crate::term::ToTermString;
use clap::ValueEnum;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Cell {
    Bit,
    Block,
    Emoji,
    Dye,
    RandomDye,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value().unwrap().get_name().fmt(f)
    }
}

impl FromStr for Cell {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

pub fn style_cell_choices(default: &str) -> String {
    let mut choices: Vec<String> = Vec::with_capacity(5);

    for &variant in Cell::value_variants() {
        let variant_s = variant.to_possible_value().unwrap().get_name().to_string();
        let mut variant_t = variant_s.to_tstr().ljust(11).set_bold();

        if variant_s == default {
            variant_t += " <- ".to_tstr().set_bold_dim().set_color("green").unwrap()
                + "default".to_tstr().set_bold_dim();
        }

        choices.push(format!("  - {variant_t}"));
    }

    format!("{}", choices.join("\n"))
}

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

pub fn style_color_choices(default: &str) -> String {
    let mut choices: Vec<String> = Vec::with_capacity(8);

    for &variant in Color::value_variants() {
        let variant_s = variant.to_possible_value().unwrap().get_name().to_string();
        let mut variant_t = "  " + variant_s.to_tstr().ljust(9);
        variant_t = variant_t.set_bold().set_color_bg(&variant_s).unwrap();

        if variant == Color::White {
            variant_t = variant_t.set_color("black").unwrap();
        }

        if variant_s == default {
            variant_t += " <- ".to_tstr().set_bold_dim().set_color("green").unwrap()
                + "default".to_tstr().set_bold_dim();
        }

        choices.push(format!("  - {variant_t}"));
    }

    format!("{}", choices.join("\n"))
}
