from random import choice, sample
from typing import Self

import numpy as np

from .biosquare import Biased, Matrix
from .term import BACKGROUND, RESET, set_bold, set_bold_dim, set_color


def _lensfilter(matrix: Matrix, sym_alive: str, sym_dead: str) -> Biased:
    film = np.where(matrix, sym_alive, sym_dead)
    return (''.join(row) for row in film)


def digitize(matrix: Matrix) -> Biased:
    """
    Outputs full-width digits.
    """
    sym_alive = set_bold('１')
    sym_dead = set_bold_dim('０')
    return (
        set_color(row, 'green')
        for row in _lensfilter(matrix, sym_alive, sym_dead)
    )


def blockify(matrix: Matrix) -> Biased:
    sym_alive = '██'
    sym_dead = '  '
    return _lensfilter(matrix, sym_alive, sym_dead)


def emojify(matrix: Matrix) -> Biased:
    sym_alive = choice('😄😁😆🤣😊🥰😍😘😚🤗🤭😋🤤🥵🥳😳😤')
    sym_dead = choice('😅😇🤪😴🤢🤮🥶😵🤯😨😰😭😱😡😠🤬😈👿🤡👻')
    return _lensfilter(matrix, sym_alive, sym_dead)


class Dye:

    FSPACE = '　'

    def __init__(self, color_alive: str, color_dead: str) -> None:
        self.sym_alive = BACKGROUND.get(
            color_alive, BACKGROUND['white']
        ) + self.FSPACE
        self.sym_dead = BACKGROUND.get(
            color_dead, BACKGROUND['black']
        ) + self.FSPACE

    def __call__(self, matrix: Matrix) -> Biased:
        return (
            row + RESET['all']
            for row in _lensfilter(matrix, self.sym_alive, self.sym_dead)
        )

    @classmethod
    def random(cls) -> Self:
        return cls(*sample(list(BACKGROUND), 2))
