use clap::{Arg, ArgAction, ArgMatches, ValueEnum, command, value_parser};
use crossterm::style::Color as CrosstermColor;
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
            // Arg::new("genesis")
            //     .long("genesis")
            //     .value_name("GENESIS")
            //     .value_parser(value_parser!(Genesis))
            //     .default_value("random")
            //     .help("Method for world initialization"),
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
                .help("Initial population density (0.5 if out of range 0.0..=1.0)"),
            Arg::new("filter")
                .long("filter")
                .value_name("FILTER")
                .value_parser(value_parser!(Filter))
                .default_value("dye")
                .help("Filter to observe the world"),
            Arg::new("color-dead")
                .short('D')
                .long("color-dead")
                .value_name("COLOR")
                .value_parser(value_parser!(Color))
                .default_value("green")
                .help("Color for dead cells (omit if filter is not dye, random if not specified)"),
            Arg::new("color-alive")
                .short('A')
                .long("color-alive")
                .value_name("COLOR")
                .value_parser(value_parser!(Color))
                .default_value("white")
                .help("Color for alive cells (omit if filter is not dye, random if not specified)"),
            Arg::new("fps-max")
                .long("fps-max")
                .value_name("DECIMAL")
                .value_parser(value_parser!(f64))
                .default_value("60.0")
                .help("Maximum fps (default value if out of range 0.0..=f64::INFINITY)"),
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
    pub density: f64,
    pub filter: Filter,
    pub color_dead: Option<Color>,
    pub color_alive: Option<Color>,
    pub fps_max: f64,
    pub show_stats: bool,
}

impl Args {
    pub fn parse() -> Self {
        const REASON: &str = "default ensures there is always a value";

        let nrows = MATCHES.get_one::<usize>("nrows").copied().expect(REASON);
        let ncols = MATCHES.get_one::<usize>("ncols").copied().expect(REASON);
        let seed = MATCHES.get_one::<String>("seed").map(Deref::deref);
        let density = MATCHES.get_one::<f64>("density").copied().expect(REASON);
        let filter = MATCHES.get_one::<Filter>("filter").copied().expect(REASON);
        let color_dead = MATCHES.get_one::<Color>("color-dead").copied();
        let color_alive = MATCHES.get_one::<Color>("color-alive").copied();
        let fps_max = MATCHES.get_one::<f64>("fps-max").copied().expect(REASON);
        let show_stats = MATCHES.get_flag("show-stats");

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
pub enum Genesis {
    Random,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Filter {
    Bit,
    Block,
    Dye,
    Emoji,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl From<Color> for CrosstermColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => CrosstermColor::Black,
            Color::Red => CrosstermColor::Red,
            Color::Green => CrosstermColor::Green,
            Color::Yellow => CrosstermColor::Yellow,
            Color::Blue => CrosstermColor::Blue,
            Color::Magenta => CrosstermColor::Magenta,
            Color::Cyan => CrosstermColor::Cyan,
            Color::White => CrosstermColor::White,
        }
    }
}
