import click

from ..__about__ import __version__
from ..biosquare import BioSquare
from ..genesis import DicingGod, background_radiate
from ..lensfilter import Dye, Emojify, blockify, digitize
from ..screen import Screen

CONTEXT_SETTINGS = dict(
    help_option_names=['-h', '--help'],
    default_map={
        'nrows': 40,
        'ncols': 40,
        'fps_max': 24,
        'cell': 'random-dye',
        'seed': None,
    }
)


@click.command(context_settings=CONTEXT_SETTINGS, epilog='Written by Lingxuan Ye <love@lingxuan.io>.')
@click.version_option(version=__version__, prog_name='LifeGame')
@click.option('-r', '--nrows', type=click.IntRange(min=1), show_default=True, help='Number of rows.')
@click.option('-c', '--ncols', type=click.IntRange(min=1), show_default=True, help='Number of columns.')
@click.option('--fps-max', type=click.IntRange(min=1), show_default=True, help='Upper limit of fps.')
@click.option('--cell', type=click.Choice(['binary', 'block', 'emoji', 'random-dye'], case_sensitive=False), help='Specifies cell style.')
@click.option('--seed', type=int, help='World initialization seed.')
def lifegame(
    nrows: int,
    ncols: int,
    fps_max: int,
    cell: str,
    seed: int,
):
    match cell:
        case 'binary':
            lensfilter = digitize
        case 'block':
            lensfilter = blockify
        case 'emoji':
            lensfilter = Emojify.random()
        case 'random-dye':
            lensfilter = Dye.random()
    biosquare = BioSquare(nrows, ncols, DicingGod(seed), lensfilter)
    screen = Screen(biosquare, fps_max=fps_max)
    screen.play()
