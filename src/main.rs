use self::cli::Args;
use self::genesis::Random;
use self::signal::setup_listener;
use self::tui::Tui;
use anyhow::Result;
use eoe::QuitOnError;
use matreex::Shape;
use rand_chacha::ChaCha8Rng;
use std::io::stdout;

mod biosquare;
mod bounded;
mod cli;
mod filter;
mod genesis;
mod signal;
mod tui;

fn run() -> Result<()> {
    setup_listener();

    let args = Args::parse();
    let shape = Shape::new(args.nrows, args.ncols);
    let genesis = Random::new()
        .density(args.density)
        .seed(args.seed)
        .generate::<ChaCha8Rng>(shape)?;
    let output = stdout().lock();
    Tui::new(genesis, args.fps_max, args.show_stats, args.filter, output).run()?;

    Ok(())
}

fn main() {
    run().quit_on_error();
}
