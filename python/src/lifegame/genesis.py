import numpy as np

from .biosquare import Matrix


class DicingGod:
    def __init__(self, seed: int | None = None) -> None:
        self.__rng = np.random.default_rng(seed)
        self.__seed = seed

    @property
    def seed(self) -> int | None:
        return self.__seed

    def __call__(self, nrows: int, ncols: int) -> Matrix:
        return self.__rng.integers(0, 2, size=(nrows, ncols), dtype=np.bool_)


def background_radiate(nrows: int, ncols: int) -> Matrix:
    return np.empty((nrows, ncols), dtype=np.bool_)
