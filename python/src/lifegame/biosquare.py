from typing import Any, Callable, Generator, Self

import numpy as np
from scipy.signal import convolve2d

Generation = np.ndarray[Any, np.dtype[np.bool_]]
Biased = Generator[str, None, None]
WorldCreator = Callable[[int, int], Generation]
LensFilter = Callable[[Generation], Biased]


class BioSquare:
    KERNEL = np.array([[1, 1, 1], [1, -9, 1], [1, 1, 1]], dtype=np.int8)

    def __init__(
        self,
        nrows: int,
        ncols: int,
        world_creator: WorldCreator,
        lensfilter: LensFilter,
    ) -> None:
        self.matrix = world_creator(nrows, ncols)
        self.creator = world_creator
        self.lensfilter = lensfilter

    def generate(self) -> Self:
        result = convolve2d(
            self.matrix,
            self.KERNEL,
            mode="same",
            boundary="wrap",
        )
        self.matrix[result == 3] = True
        self.matrix[(result != -6) & (result != -7) & (result < 0)] = False
        return self

    def observe(self) -> Biased:
        return self.lensfilter(self.matrix)

    def reset(self) -> Self:
        self.matrix = self.creator(*self.matrix.shape)
        return self
