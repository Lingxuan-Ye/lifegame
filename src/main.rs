use self::cli::Args;
use self::cli::Filter;
use self::error::QuitOnError;
use self::filter::{Bit, Block, Dye, Emoji};
use self::genesis::Random;
use self::signal::setup_listener;
use self::tui::Tui;
use anyhow::Result;
use matreex::Shape;
use rand_chacha::ChaCha8Rng;
use std::io::stdout;

mod biosquare;
mod cell;
mod cli;
mod error;
mod filter;
mod genesis;
mod signal;
mod timer;
mod tui;

fn execute() -> Result<()> {
    setup_listener();

    let args = Args::parse();

    let shape = Shape::new(args.nrows, args.ncols);
    let genesis = Random::new()
        .density(args.density)?
        .seed(args.seed)
        .generate::<ChaCha8Rng>(shape)?;
    let output = stdout().lock();

    let mut tui = Tui::new(genesis, output);

    if (0.0..=f64::INFINITY).contains(&args.fps_max) {
        tui.set_fps_max(args.fps_max)?;
    }

    if args.show_stats {
        tui.show_stats();
    } else {
        tui.hide_stats();
    }

    match args.filter {
        Filter::Bit => {
            tui.run(&Bit)?;
        }
        Filter::Block => {
            tui.run(&Block)?;
        }
        Filter::Dye => {
            let filter = Dye::new(args.color_dead, args.color_alive);
            tui.run(&filter)?;
        }
        Filter::Emoji => {
            let filter = Emoji::random();
            tui.run(&filter)?;
        }
    }

    Ok(())
}

fn main() {
    execute().quit_on_error();
}
