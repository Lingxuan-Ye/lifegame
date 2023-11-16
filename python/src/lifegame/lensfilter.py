import random
from abc import ABC, abstractmethod
from typing import Self

import numpy as np

from .biosquare import Biased, Generation
from .term import ESCSEQ, TermString


class _LensFilter(ABC):
    """
    Using attributes may slightly reduce string instantiation overhead,
    or may not.
    """

    sym_alive: str
    sym_dead: str

    def _project(self, gen: Generation) -> Biased:
        image = np.where(gen, self.sym_alive, self.sym_dead)
        return ("".join(row) for row in image)

    @abstractmethod
    def observe(self, gen: Generation) -> Biased:
        pass


class Digitize(_LensFilter):
    """
    Outputs full-width binary digits.
    """

    sym_alive = str(TermString("１").set_bold())
    sym_dead = str(TermString("０").set_bold_dim())

    def observe(self, gen: Generation) -> Biased:
        for row in self._project(gen):
            yield str(TermString(row).set_color("green"))

    @classmethod
    def ascii_compatible(cls) -> Self:
        inst = cls()
        inst.sym_alive = str(TermString("1").set_bold())
        inst.sym_dead = str(TermString("0").set_bold())
        return inst


class Blockify(_LensFilter):
    sym_alive = "██"
    sym_dead = "  "

    def observe(self, gen: Generation) -> Biased:
        return self._project(gen)


class Emojify(_LensFilter):
    def __init__(self, sym_alive: str, sym_dead: str) -> None:
        self.sym_alive = sym_alive
        self.sym_dead = sym_dead

    def observe(self, gen: Generation) -> Biased:
        return self._project(gen)

    @classmethod
    def random(cls, seed: int | None = None) -> Self:
        random.seed(seed)
        sym_alive = random.choice("😆🤣😊🥰😍🤗🤭😋🤤😤")
        sym_dead = random.choice("🤢🥶🥵😡🤬😈👿🤡👻")
        return cls(sym_alive, sym_dead)


class Dye(_LensFilter):
    FSPACE = "  "

    def __init__(self, color_alive: str, color_dead: str) -> None:
        self.sym_alive = ESCSEQ["background"][color_alive] + self.FSPACE
        self.sym_dead = ESCSEQ["background"][color_dead] + self.FSPACE

    def observe(self, gen: Generation) -> Biased:
        for row in self._project(gen):
            yield row + ESCSEQ["reset"]["all"]

    @classmethod
    def random(cls, seed: int | None = None) -> Self:
        random.seed(seed)
        return cls(*random.sample(list(ESCSEQ["background"]), 2))
