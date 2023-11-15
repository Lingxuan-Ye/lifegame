import numpy as np

from .biosquare import Matrix


class DicingGod:
    def __init__(self, seed: int | None = None) -> None:
        self.__seed = seed

    @property
    def seed(self) -> int | None:
        return self.__seed

    def __call__(self, nrows: int, ncols: int) -> Matrix:
        rng = np.random.Generator(np.random.PCG64(self.seed))
        return rng.integers(0, 2, size=(nrows, ncols), dtype=np.bool_)
