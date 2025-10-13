use crate::bounded::Bounded;
use crate::genesis::Density;
use crate::tui::FpsMax;
use clap::{Arg, ArgAction, ArgMatches, ValueEnum, command, value_parser};
use crossterm::style::Color;
use std::ops::Deref;
use std::sync::LazyLock;

static MATCHES: LazyLock<ArgMatches> = LazyLock::new(|| {
    command!()
        .args([
            Arg::new("nrows")
                .short('r')
                .long("nrows")
                .value_name("INTEGER")
                .value_parser(value_parser!(usize))
                .default_value("40")
                .help("Number of rows"),
            Arg::new("ncols")
                .short('c')
                .long("ncols")
                .value_name("INTEGER")
                .value_parser(value_parser!(usize))
                .default_value("40")
                .help("Number of columns"),
            Arg::new("seed")
                .long("seed")
                .value_name("STRING")
                .help("Seed for world initialization"),
            Arg::new("density")
                .short('p')
                .long("density")
                .value_name("DECIMAL")
                .value_parser(value_parser!(f64))
                .default_value("0.5")
                .help(format!(
                    "Initial population density (use default if out of range {:?})",
                    Density::RANGE
                )),
            Arg::new("filter")
                .long("filter")
                .value_name("FILTER")
                .value_parser(value_parser!(FilterKind))
                .default_value("dye")
                .hide_default_value(true)
                .hide_possible_values(true)
                .help(FilterKind::help(
                    "Filter to observe the world",
                    Some(FilterKind::Dye),
                )),
            Arg::new("color-dead")
                .short('D')
                .long("color-dead")
                .value_name("COLOR")
                .value_parser(value_parser!(ColorKind))
                .default_value("green")
                .hide_default_value(true)
                .hide_possible_values(true)
                .help(ColorKind::help(
                    "Color for dead cells (omit if filter is not dye)",
                    Some(ColorKind::Green),
                )),
            Arg::new("color-alive")
                .short('A')
                .long("color-alive")
                .value_name("COLOR")
                .value_parser(value_parser!(ColorKind))
                .default_value("white")
                .hide_default_value(true)
                .hide_possible_values(true)
                .help(ColorKind::help(
                    "Color for alive cells (omit if filter is not dye)",
                    Some(ColorKind::White),
                )),
            Arg::new("fps-max")
                .long("fps-max")
                .value_name("DECIMAL")
                .value_parser(value_parser!(f64))
                .default_value("60.0")
                .help(format!(
                    "Maximum fps (use default if out of range {:?})",
                    FpsMax::RANGE
                )),
            Arg::new("show-stats")
                .long("show-stats")
                .action(ArgAction::SetTrue)
                .help("Show statistics"),
        ])
        .get_matches()
});

#[derive(Clone, Debug)]
pub struct Args {
    pub nrows: usize,
    pub ncols: usize,
    pub seed: Option<&'static str>,
    pub density: Density,
    pub filter: FilterKind,
    pub color_dead: Color,
    pub color_alive: Color,
    pub fps_max: FpsMax,
    pub show_stats: bool,
}

impl Args {
    pub fn parse() -> Self {
        let nrows = MATCHES.get_one::<usize>("nrows").copied().unwrap();
        let ncols = MATCHES.get_one::<usize>("ncols").copied().unwrap();
        let seed = MATCHES.get_one::<String>("seed").map(Deref::deref);
        let density = MATCHES.get_one::<f64>("density").copied().unwrap();
        let filter = MATCHES.get_one::<FilterKind>("filter").copied().unwrap();
        let color_dead = MATCHES
            .get_one::<ColorKind>("color-dead")
            .copied()
            .unwrap()
            .into();
        let color_alive = MATCHES
            .get_one::<ColorKind>("color-alive")
            .copied()
            .unwrap()
            .into();
        let fps_max = MATCHES.get_one::<f64>("fps-max").copied().unwrap();
        let show_stats = MATCHES.get_flag("show-stats");

        let density = Density::new_or_default(density);
        let fps_max = FpsMax::new_or_default(fps_max);

        Self {
            nrows,
            ncols,
            seed,
            density,
            filter,
            color_dead,
            color_alive,
            fps_max,
            show_stats,
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum FilterKind {
    Bit,
    Block,
    Dye,
    Emoji,
    Hanzi,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ColorKind {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl From<ColorKind> for Color {
    fn from(color: ColorKind) -> Self {
        match color {
            ColorKind::Black => Color::Black,
            ColorKind::Red => Color::Red,
            ColorKind::Green => Color::Green,
            ColorKind::Yellow => Color::Yellow,
            ColorKind::Blue => Color::Blue,
            ColorKind::Magenta => Color::Magenta,
            ColorKind::Cyan => Color::Cyan,
            ColorKind::White => Color::White,
        }
    }
}

trait ValueEnumExt: ValueEnum {
    fn help(description: &str, default: Option<Self>) -> String {
        let mut message = description.to_string();
        if let Some(default) = default.and_then(|variant| variant.to_possible_value()) {
            message.push_str(&format!(" [default: {}]", default.get_name()));
        }
        Self::value_variants()
            .iter()
            .filter_map(|variant| variant.to_possible_value())
            .for_each(|entry| {
                message.push_str(&format!("\n- {}", entry.get_name()));
            });
        message
    }
}

impl<T> ValueEnumExt for T where T: ValueEnum {}
