import click

from .. import genesis, lensfilter
from ..__about__ import __version__
from ..biosquare import BioSquare, LensFilter
from ..screen import Screen
from ..term import TermString

CELL = [
    "ascii-bit",
    "bit",
    "block",
    "emoji",
    "dye",
    "random-dye",
]

COLOR = [
    "red",
    "yellow",
    "green",
    "cyan",
    "blue",
    "magenta",
    "black",
    "white",
]

DEFAULT_TAG = TermString(" <= ").set_color("green") + "default"


def style_cell_choices(default: str) -> str:
    choices: list[str] = []
    for variant in CELL:
        variant_t = TermString(f"{variant:<11}").set_bold()
        if variant == default:
            variant_t += DEFAULT_TAG
        choices.append(f"  - {variant_t}")
    return "\n".join(choices)


def style_color_choices(default: str) -> str:
    choices: list[str] = []
    for variant in COLOR:
        variant_t = (
            TermString(f"  {variant:<9}").set_bold().set_color_bg(variant)
        )
        if variant == "white":
            variant_t.set_color("black")
        if variant == default:
            variant_t += DEFAULT_TAG
        choices.append(f"  - {variant_t}")
    return "\n".join(choices)


@click.command(
    context_settings=dict(
        help_option_names=["-h", "--help"],
    )
)
@click.option(
    "-r",
    "--nrows",
    metavar="INTEGER",
    type=click.IntRange(min=0),
    default=32,
    show_default=True,
    help="Number of rows.",
)
@click.option(
    "-c",
    "--ncols",
    metavar="INTEGER",
    type=click.IntRange(min=0),
    default=32,
    show_default=True,
    help="Number of columns.",
)
@click.option(
    "--seed",
    type=int,
    metavar="INTEGER",
    help="Seed for world initialization.",
)
@click.option(
    "-p",
    "--density",
    type=float,
    metavar="DECIMAL",
    default=0.5,
    help="The initial population density.",
)
@click.option(
    "--cell",
    metavar="CHOICE",
    type=click.Choice(
        CELL,
        case_sensitive=False,
    ),
    default="dye",
    help="\b\nSpecify cell style.\n" + style_cell_choices("dye"),
)
@click.option(
    "-A",
    "--color-alive",
    metavar="CHOICE",
    type=click.Choice(
        COLOR,
        case_sensitive=False,
    ),
    default="white",
    help="\b\nColor for alive cells, valid when `--cell=dye'.\n"
    + style_color_choices("white"),
)
@click.option(
    "-D",
    "--color-dead",
    type=click.Choice(
        COLOR,
        case_sensitive=False,
    ),
    default="green",
    help="\b\nColor for alive cells, valid when `--cell=dye'.\n"
    + style_color_choices("green"),
)
@click.option(
    "-i",
    "--iteration-max",
    metavar="INTEGER",
    type=int,
    help="Set maximum iterations; Run forever if not given.",
)
@click.option(
    "--fps-max",
    metavar="DECIMAL",
    type=click.FloatRange(min=0.0, min_open=True),
    default=24.0,
    show_default=True,
    help="Set maximum fps.",
)
@click.option(
    "--show-stats/--hide-stats",
    default=True,
    help="Show/hide statistics.",
)
@click.version_option(
    __version__,
    "-V",
    "--version",
)
def lifegame(
    nrows: int,
    ncols: int,
    seed: int,
    density: float,
    cell: str,
    color_alive: str,
    color_dead: str,
    iteration_max: int,
    fps_max: int,
    show_stats: bool,
):
    """
    \b\n
    A simple Python implementation of the classic cellular automaton,
    Conway's Game of Life.
    """
    filter: LensFilter
    match cell:
        case "ascii-bit":
            filter = lensfilter.Digitize.ascii_compatible()
        case "bit":
            filter = lensfilter.Digitize()
        case "block":
            filter = lensfilter.Blockify()
        case "emoji":
            filter = lensfilter.Emojify.random()
        case "dye":
            filter = lensfilter.Dye(color_alive, color_dead)
        case "random-dye":
            filter = lensfilter.Dye.random()

    biosquare = BioSquare(
        nrows, ncols, genesis.DicingGod(seed, initial_density=density), filter
    )

    screen = Screen(
        biosquare,
        iterno_max=iteration_max,
        fps_max=fps_max,
        show_stats=show_stats,
    )

    screen.play()
