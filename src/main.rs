use self::cli::{Args, Filter};
use self::error::QuitOnError;
use self::filter::{Bit, Block, Dye, Emoji, Hanzi};
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

fn run() -> Result<()> {
    setup_listener();

    let args = Args::parse();
    let shape = Shape::new(args.nrows, args.ncols);
    let genesis = Random::new()
        .density(args.density)?
        .seed(args.seed)
        .generate::<ChaCha8Rng>(shape)?;
    let output = stdout().lock();

    match args.filter {
        Filter::Bit => {
            let filter = Bit;
            Tui::new(genesis, args.fps_max, args.show_stats, filter, output).run()?;
        }
        Filter::Block => {
            let filter = Block;
            Tui::new(genesis, args.fps_max, args.show_stats, filter, output).run()?;
        }
        Filter::Dye => {
            let filter = Dye::new(args.color_dead, args.color_alive);
            Tui::new(genesis, args.fps_max, args.show_stats, filter, output).run()?;
        }
        Filter::Emoji => {
            let filter = Emoji::random();
            Tui::new(genesis, args.fps_max, args.show_stats, filter, output).run()?;
        }
        Filter::Hanzi => {
            let filter = Hanzi;
            Tui::new(genesis, args.fps_max, args.show_stats, filter, output).run()?;
        }
    }

    Ok(())
}

fn main() {
    run().quit_on_error();
}
