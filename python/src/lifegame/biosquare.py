from typing import Any, Generator, Protocol, Self

import numpy as np
from scipy.signal import convolve2d  # type: ignore

Generation = np.ndarray[Any, np.dtype[np.bool_]]
Biased = Generator[str, None, None]


class WorldCreator(Protocol):
    def create(self, nrows: int, ncols: int) -> Generation:
        pass


class LensFilter(Protocol):
    def observe(self, gen: Generation) -> Biased:
        pass


class BioSquare:
    KERNEL = np.array([[1, 1, 1], [1, -9, 1], [1, 1, 1]], dtype=np.int8)

    def __init__(
        self,
        nrows: int,
        ncols: int,
        world_creator: WorldCreator,
        lensfilter: LensFilter,
    ) -> None:
        self.generation = world_creator.create(nrows, ncols)
        self.creator = world_creator
        self.lensfilter = lensfilter

    def generate(self) -> Self:
        result: np.ndarray = convolve2d(
            self.generation,
            self.KERNEL,
            mode="same",
            boundary="wrap",
        )
        self.generation[result == 3] = True
        self.generation[(result != -6) & (result != -7) & (result < 0)] = False
        return self

    def observe(self) -> Biased:
        return self.lensfilter.observe(self.generation)

    def population_density(self) -> float:
        return self.generation.sum() / self.generation.size

    def reset(self) -> Self:
        self.generation = self.creator.create(*self.generation.shape)
        return self
