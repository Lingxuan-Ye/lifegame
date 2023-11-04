import signal
from typing import Any, Generator, NamedTuple

from .biosquare import BioSquare
from .term import TermString, erase_screen, reset_cursor, set_bold, set_color
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
        fps_max: float = 24.0,
        style: Style = Style(
            x_offset=2,
            y_offset=1,
            section_sep=2,
            label_width=20,
            value_width=40
        ),
    ) -> None:
        self.iterno = 0
        self.timer = Timer()
        self.biosquare = biosquare
        self.style = style
        self.fps_max = fps_max

    @property
    def fps(self) -> float:
        try:
            fps = self.timer.NS_PER_S / self.timer.check_delta()
        except ZeroDivisionError:
            fps = float('inf')
        return fps

    @property
    def seperator(self) -> Rows:
        for _ in range(self.style.section_sep):
            yield ''

    @property
    def exit_message(self) -> Rows:
        yield set_color(set_bold('GAME OVER'), 'green')

    def _meas_fmt(self, label: Any, value: Any) -> str:
        label_t = TermString(set_bold(str(label)), esc_len=9)
        label_s = label_t.ljust(self.style.label_width).to_str()
        if isinstance(value, float):
            value = f'{value:.2f}'
        value_t = TermString(value)
        value_s = value_t.rjust(self.style.value_width).to_str()
        return label_s + value_s

    def observe(self) -> Rows:
        yield self._meas_fmt('Iteration', self.iterno)
        yield self._meas_fmt('FPS', self.fps)
        yield self._meas_fmt('Runtime', self.timer.check_fmt(record=True))

    def render(self, is_last_frame: bool = False) -> Rows:
        for row in self.biosquare.observe():
            yield row
        for row in self.seperator:
            yield row
        for row in self.observe():
            yield row
        if not is_last_frame:
            return
        for row in self.seperator:
            yield row
        for row in self.exit_message:
            yield row

    def offset(self, frame: Rows) -> Rows:
        for _ in range(self.style.y_offset):
            yield ''
        for row in frame:
            yield ' ' * self.style.x_offset + row

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

        frame_duration_min = self.timer.NS_PER_S / self.fps_max
        erase_screen()
        self.timer.reset()

        while not recv_sigint:
            start = self.timer.check()
            self.display()
            self.biosquare.generate()
            self.iterno += 1
            while self.timer.check() - start < frame_duration_min:
                pass

        self.display(is_last_frame=True)
