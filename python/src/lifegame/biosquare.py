from typing import Any, Callable, Generator, Self

import numpy as np
from scipy.signal import convolve2d

Matrix = np.ndarray[Any, np.dtype[np.bool_]]
Biased = Generator[str, None, None]
WorldCreator = Callable[[int, int], Matrix]
LensFilter = Callable[[Matrix], Biased]


class BioSquare:

    KERNEL = np.array([[1, 1, 1], [1, -9, 1], [1, 1, 1]], dtype=np.int8)

    def __init__(
        self, nrows: int, ncols: int, world_creator: WorldCreator,
        lensfilter: LensFilter
    ) -> None:
        self.__current = world_creator(nrows, ncols)
        self.__next = self.__current.copy()
        self.creator = world_creator
        self.lensfilter = lensfilter

    def generate(self) -> Self:
        result = convolve2d(
            self.__current, self.KERNEL, mode='same', boundary='wrap'
        )
        self.__next[result == 3] = True
        self.__next[(result != -6) & (result != -7) & (result < 0)] = False
        self.__current[:] = self.__next
        return self

    def observe(self) -> Biased:
        return self.lensfilter(self.__current)

    def reset(self) -> Self:
        self.__current = self.creator(*self.__current.shape)
        return self
