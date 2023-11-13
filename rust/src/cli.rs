pub mod choices;

use clap::{command, value_parser, Arg, ArgAction, ArgGroup, Command};

pub fn build() -> Command {
    command!().args([
        Arg::new("nrows")
            .short('r')
            .long("nrows")
            .value_name("INTEGER")
            .value_parser(value_parser!(usize))
            .default_value("32")
            .help("Number of rows"),
        Arg::new("ncols")
            .short('c')
            .long("ncols")
            .value_name("INTEGER")
            .value_parser(value_parser!(usize))
            .default_value("32")
            .help("Number of columns"),
        Arg::new("seed")
            .long("seed")
            .value_name("INTEGER")
            .value_parser(value_parser!(u64))
            .help("Seed for world initialization"),
        Arg::new("density")
            .short('p')
            .long("density")
            .value_name("DECIMAL")
            .value_parser(|s: &str| -> Result<f64, String> {
                let f: f64 = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
                if f.is_nan() {
                    Err(
                        "value cannot be NaN, stop testing thoes strange corner cases!".to_string(),
                    )?
                }
                if f.is_sign_negative() {
                    Err("value cannot be negative".to_string())?
                }
                if f > 1.0 {
                    Err("value cannot be greater than 1".to_string())?
                }
                Ok(f)
            })
            .default_value("0.5")
            .help("The initial population density"),
        Arg::new("cell")
            .long("cell")
            .value_name("CHOICE")
            .value_parser(value_parser!(choices::Cell))
            .default_value("dye")
            .hide_default_value(true)
            .hide_possible_values(true)
            .help(format!(
                "Specify cell style\n{choices}",
                choices = choices::style_cell_choices("dye")
            )),
        Arg::new("color-alive")
            .short('A')
            .long("color-alive")
            .value_name("CHOICE")
            .value_parser(value_parser!(choices::Color))
            .default_value("white")
            .hide_default_value(true)
            .hide_possible_values(true)
            .help(format!(
                "Color for alive cells, valid when `--cell=dye'\n{choices}",
                choices = choices::style_color_choices("white")
            )),
        Arg::new("color-dead")
            .short('D')
            .long("color-dead")
            .value_name("CHOICE")
            .value_parser(value_parser!(choices::Color))
            .default_value("green")
            .hide_default_value(true)
            .hide_possible_values(true)
            .help(format!(
                "Color for dead cells, valid when `--cell=dye'\n{choices}",
                choices = choices::style_color_choices("green")
            )),
        Arg::new("iteration-max")
            .short('i')
            .long("iteration-max")
            .value_name("INTEGER")
            .value_parser(value_parser!(usize))
            .help("Set maximum iterations; Run forever if not given"),
        Arg::new("fps-max")
            .long("fps-max")
            .value_name("DECIMAL")
            .value_parser(|s: &str| -> Result<f64, String> {
                let f: f64 = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
                if f.is_nan() {
                    Err(
                        "value cannot be NaN, stop testing thoes strange corner cases!".to_string(),
                    )?
                }
                if f.is_sign_negative() {
                    Err("value cannot be negative".to_string())?
                }
                Ok(f)
            })
            .default_value("24.0")
            .help("Set maximum fps"),
        Arg::new("hide-stats")
            .long("hide-stats")
            .action(ArgAction::SetTrue)
            .help("Hide statistics"),
    ])
}
