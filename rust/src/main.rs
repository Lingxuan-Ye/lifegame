use lifegame::cli;
use lifegame::cli::choices::{Cell, Color};
use lifegame::genesis;
use lifegame::lensfilter;
use lifegame::{BioSquare, LensFilter, Screen};

fn main() {
    let matches = cli::build().get_matches();

    let &nrows = matches.get_one::<usize>("nrows").unwrap();
    let &ncols = matches.get_one::<usize>("ncols").unwrap();

    let seed = matches.get_one::<u64>("seed").map(|&u| u);
    let &population_density = matches.get_one::<f64>("density").unwrap();
    let world_creator = Box::new(genesis::DicingGod::new(seed, population_density));

    let lensfilter: Box<dyn LensFilter> = match matches.get_one::<Cell>("cell").unwrap() {
        Cell::Bit => Box::new(lensfilter::Digitize::new()),
        Cell::Block => Box::new(lensfilter::Blockify::new()),
        Cell::Emoji => Box::new(lensfilter::Emojify::random()),
        Cell::Dye => {
            let color_alive = matches.get_one::<Color>("color-alive").unwrap().to_string();
            let color_dead = matches.get_one::<Color>("color-dead").unwrap().to_string();
            Box::new(lensfilter::Dye::new(&color_alive, &color_dead))
        }
        Cell::RandomDye => Box::new(lensfilter::Dye::random()),
    };

    let biosquare = BioSquare::new(nrows, ncols, world_creator, lensfilter);
    let mut screen = Screen::new(biosquare);

    let iteration_max = matches.get_one::<usize>("iteration-max").map(|&u| u);
    let fps_max = matches.get_one::<f64>("fps-max").map(|&f| f).unwrap();

    screen
        .set_iterno_max(iteration_max)
        .set_fps_max(fps_max)
        .unwrap();

    if matches.get_flag("hide-stats") {
        screen.hide_stats();
    } else {
        screen.show_stats();
    }

    screen.play();
}
