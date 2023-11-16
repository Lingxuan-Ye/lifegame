import math
import signal
from typing import Generator, NamedTuple

from .biosquare import BioSquare
from .term import TermString, erase_screen, reset_cursor
from .timer import Timer

Rows = Generator[str, None, None]


class Style(NamedTuple):
    x_offset: int
    y_offset: int
    section_sep: int
    label_width: int
    value_width: int


class Screen:
    def __init__(
        self,
        biosquare: BioSquare,
        *,
        iterno_max: int | None = None,
        fps_max: float = float("inf"),
        show_stats: bool = True,
        style: Style = Style(
            x_offset=2,
            y_offset=1,
            section_sep=2,
            label_width=20,
            value_width=40,
        ),
    ) -> None:
        self.biosquare = biosquare
        self.timer = Timer()
        self.iterno = 0
        self.iterno_max = iterno_max
        self.fps_max = fps_max
        self.show_stats = show_stats
        self.style = style

    @property
    def fps_max(self) -> float:
        return self.__fps_max

    @fps_max.setter
    def fps_max(self, value: float) -> None:
        if math.isnan(value):
            raise ValueError("value cannot be NaN")
        if value < 0:
            raise ValueError("value cannot be negative")
        self.__fps_max = value

    @property
    def seperator(self) -> Rows:
        for _ in range(self.style.section_sep):
            yield ""

    def _measurement_fmt(self, label: TermString, value: TermString) -> str:
        label_s = str(label.set_bold().ljust(self.style.label_width))
        value_s = str(value.rjust(self.style.value_width))
        return label_s + value_s

    def observe(self) -> Rows:
        density = self.biosquare.population_density()
        try:
            fps = self.timer.NANOS_PER_SEC / self.timer.check_delta()
        except ZeroDivisionError:
            fps = float("inf")

        yield self._measurement_fmt(
            TermString("Iteration"), TermString(self.iterno)
        )

        yield self._measurement_fmt(
            TermString("Population Density"),
            TermString(f"{density*100:.2f} %"),
        )

        yield self._measurement_fmt(
            TermString("FPS"), TermString(f"{fps:.2f}")
        )

        yield self._measurement_fmt(
            TermString("Runtime"), self.timer.check_fmt(True)
        )

    @property
    def exit_message(self) -> Rows:
        yield str(TermString("GAME OVER").set_bold().set_color("green"))

    def render(self, is_last_frame: bool = False) -> Rows:
        for row in self.biosquare.observe():
            yield row

        if self.show_stats:
            for row in self.seperator:
                yield row
            for row in self.observe():
                yield row

        if is_last_frame:
            for row in self.seperator:
                yield row
            for row in self.exit_message:
                yield row

    def offset(self, frame: Rows) -> Rows:
        for _ in range(self.style.y_offset):
            yield ""
        for row in frame:
            yield " " * self.style.x_offset + row

    def display(self, is_last_frame: bool = False) -> None:
        reset_cursor()
        for row in self.offset(self.render(is_last_frame)):
            print(row)

    def play(self) -> None:
        recv_sigint = False

        def exit_handler(sifnum, frame):
            nonlocal recv_sigint
            recv_sigint = True

        signal.signal(signal.SIGINT, exit_handler)

        frame_duration_min = self.timer.NANOS_PER_SEC / self.fps_max

        erase_screen()
        self.timer.reset()

        while not recv_sigint:
            if self.iterno_max is not None and self.iterno > self.iterno_max:
                break

            start = self.timer.check()
            self.display()
            self.biosquare.generate()
            self.iterno += 1

            while self.timer.check() - start < frame_duration_min:
                if recv_sigint:
                    break

        self.display(is_last_frame=True)
