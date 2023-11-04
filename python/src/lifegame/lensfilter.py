import random
from abc import ABC, abstractmethod
from typing import Self

import numpy as np

from .biosquare import Biased, Matrix
from .term import BACKGROUND, RESET, set_bold, set_bold_dim, set_color


class _LensFilter(ABC):
    """
    Using attributes may slightly reduce string instantiation overhead,
    or may not.
    """
    sym_alive: str
    sym_dead: str

    def _lensfilter(self, matrix: Matrix) -> Biased:
        film = np.where(matrix, self.sym_alive, self.sym_dead)
        return ("".join(row) for row in film)

    @abstractmethod
    def __call__(self, matrix: Matrix) -> Biased:
        pass


class Digitize(_LensFilter):
    """
    Outputs full-width binary digits.
    """

    sym_alive = str(set_bold("１"))
    sym_dead = str(set_bold_dim("０"))

    def __call__(self, matrix: Matrix) -> Biased:
        for row in self._lensfilter(matrix):
            yield str(set_color(row, "green"))


class Blockify(_LensFilter):
    sym_alive = "██"
    sym_dead = "  "

    def __call__(self, matrix: Matrix) -> Biased:
        return self._lensfilter(matrix)


class Emojify(_LensFilter):
    def __init__(self, sym_alive: str, sym_dead: str) -> None:
        self.sym_alive = sym_alive
        self.sym_dead = sym_dead

    def __call__(self, matrix: Matrix) -> Biased:
        return self._lensfilter(matrix)

    @classmethod
    def random(cls, seed: int | None = None) -> Self:
        random.seed(seed)
        sym_alive = random.choice("😄😁😆🤣😊🥰😍😘😚🤗🤭😋🤤🥳😳😤")
        sym_dead = random.choice("🤢🥶🥵😡🤬😈👿🤡👻")
        return cls(sym_alive, sym_dead)


class Dye(_LensFilter):
    FSPACE = "　"

    def __init__(self, color_alive: str, color_dead: str) -> None:
        self.sym_alive = BACKGROUND[color_alive] + self.FSPACE
        self.sym_dead = BACKGROUND[color_dead] + self.FSPACE

    def __call__(self, matrix: Matrix) -> Biased:
        for row in self._lensfilter(matrix):
            yield row + RESET["all"]

    @classmethod
    def random(cls, seed: int | None = None) -> Self:
        random.seed(seed)
        return cls(*random.sample(list(BACKGROUND), 2))
