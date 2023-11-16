import numpy as np

from .biosquare import Generation


class DicingGod:
    def __init__(self, seed: int | None, initial_density: float) -> None:
        if initial_density < 0:
            raise ValueError("initial density cannot be negative")
        if initial_density > 1:
            raise ValueError("initial density cannot be greater than 1")
        self.seed = seed
        self.initial_density = initial_density

    def create(self, nrows: int, ncols: int) -> Generation:
        rng = np.random.Generator(np.random.PCG64(self.seed))
        init_gen = rng.random(size=(nrows, ncols)) >= self.initial_density
        return init_gen
