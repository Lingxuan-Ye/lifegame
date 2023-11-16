pub mod cell;
pub mod color;

pub use cell::Cell;
pub use color::Color;

use crate::term::{TermString, ToTermString};
use clap::ValueEnum;
use once_cell::sync::Lazy;

pub static DEFAULT_TAG: Lazy<TermString> =
    Lazy::new(|| " <= ".to_tstr().set_color("green").unwrap() + "default");

pub trait StyleChoice
where
    Self: ValueEnum + PartialEq + Eq,
{
    fn style(default: Self) -> String {
        let mut choices: Vec<String> = Vec::new();
        for variant in Self::value_variants() {
            let variant_s = variant.to_possible_value().unwrap().get_name().to_string();
            let mut variant_t = format!("{variant_s:<11}").to_tstr().set_bold();
            if *variant == default {
                variant_t += &DEFAULT_TAG as &str;
            }
            choices.push(format!("  - {variant_t}"));
        }
        choices.join("\n")
    }
}
