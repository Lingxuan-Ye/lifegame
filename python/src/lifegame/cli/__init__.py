import click

from .. import genesis, lensfilter
from ..__about__ import __version__
from ..biosquare import BioSquare, LensFilter
from ..screen import Screen

CONTEXT_SETTINGS = dict(
    help_option_names=["-h", "--help"],
    default_map={
        "nrows": 32,
        "ncols": 32,
        "cell": "dye",
        "color_alive": "white",
        "color_dead": "green",
        "iteration_max": -1,
        "fps_max": 24.0,
        "show_stats": True,
        "seed": None,
    },
)
CellChoice = click.Choice(
    [
        "bit",
        "block",
        "emoji",
        "dye",
        "random-dye",
    ],
    case_sensitive=False,
)
ColorChoice = click.Choice(
    [
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white",
    ],
    case_sensitive=False,
)


@click.command(
    context_settings=CONTEXT_SETTINGS,
    epilog="Written by Lingxuan Ye <love@lingxuan.io>.",
)
@click.option(
    "-r",
    "--nrows",
    metavar="",
    type=click.IntRange(min=1),
    help="Number of rows.",
)
@click.option(
    "-c",
    "--ncols",
    metavar="",
    type=click.IntRange(min=1),
    help="Number of columns.",
)
@click.option(
    "-C",
    "--cell",
    type=CellChoice,
    show_default=True,
    help="Specify cell style.",
)
@click.option(
    "-A",
    "--color-alive",
    type=ColorChoice,
    help="Color for alive cells. Omit if `--cell` is not set to 'dye'.",
)
@click.option(
    "-D",
    "--color-dead",
    type=ColorChoice,
    help="Color for dead Cells. Omit if `--cell` is not set to 'dye'.",
)
@click.option(
    "-i",
    "--iteration-max",
    metavar="",
    type=int,
    show_default=True,
    help="Set maximum iterations. Run forever if negative.",
)
@click.option(
    "--fps-max",
    metavar="",
    type=click.FloatRange(min=0.0, min_open=True),
    help="Set maximum fps.",
)
@click.option(
    "--show-stats/--hide-stats",
    help="Show or hide statistics.",
)
@click.option(
    "--seed",
    type=int,
    metavar="",
    help="Seed for world initialization.",
)
@click.version_option(
    __version__,
    "-V",
    "--version",
    prog_name="LifeGame",
)
def lifegame(
    nrows: int,
    ncols: int,
    cell: str,
    color_alive: str,
    color_dead: str,
    iteration_max: int,
    fps_max: int,
    show_stats: bool,
    seed: int,
):
    """
    \b
    A simple Python implementation of the classic cellular automaton,
    Conway's Game of Life.
    """
    filter: LensFilter
    match cell:
        case "bit":
            filter = lensfilter.Digitize().observe
        case "block":
            filter = lensfilter.Blockify().observe
        case "emoji":
            filter = lensfilter.Emojify.random().observe
        case "dye":
            filter = lensfilter.Dye(color_alive, color_dead).observe
        case "random-dye":
            filter = lensfilter.Dye.random().observe
    biosquare = BioSquare(nrows, ncols, genesis.DicingGod(seed), filter)
    screen = Screen(
        biosquare,
        iterno_max=iteration_max,
        fps_max=fps_max,
        show_stats=show_stats,
    )
    screen.play()
