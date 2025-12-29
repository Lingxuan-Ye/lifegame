use self::cli::Args;
use self::genesis::Genesis;
use self::screen::Screen;
use self::signal::setup_listener;
use anyhow::Result;
use eoe::QuitOnError;
use matreex::Shape;
use std::io::stdout;

mod biosquare;
mod cli;
mod filter;
mod genesis;
mod screen;
mod signal;

fn run() -> Result<()> {
    setup_listener();

    let args = Args::parse();
    let shape = Shape::new(args.nrows, args.ncols);
    let genesis = Genesis::new(shape).random(args.density, args.seed)?;
    let output = stdout().lock();

    Screen::new(genesis, args.fps_max, args.show_stats, args.filter, output)?.run()?;

    Ok(())
}

fn main() {
    run().quit_on_error();
}
