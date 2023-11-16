from typing import Any, Self

from .const import ESCSEQ


def erase_screen() -> None:
    print(ESCSEQ["erase"]["screen"], end="")


def reset_cursor() -> None:
    print(ESCSEQ["reset"]["cursor"], end="")


class TermString:
    data: str
    esc_len: int

    def __init__(self, seq: Any) -> None:
        if isinstance(seq, self.__class__):
            self.data = seq.data
            self.esc_len = seq.esc_len
            return
        if isinstance(seq, str):
            self.data = seq
        else:
            self.data = str(seq)
        self.esc_len = 0

    @classmethod
    def wrap(cls, seq: Any) -> Self:
        return cls(seq)

    @classmethod
    def __from_escseq(cls, cat: str, key: str) -> Self:
        seq = ESCSEQ[cat][key]
        inst = cls(seq)
        inst.esc_len = len(seq)
        return inst

    @classmethod
    def from_escseq(cls, cat: str, key: str) -> Self | None:
        cat_dict = ESCSEQ.get(cat)
        if cat_dict is None:
            return None
        seq = cat_dict.get(key)
        if seq is None:
            return None
        inst = cls(seq)
        inst.esc_len = len(seq)
        return inst

    def __str__(self) -> str:
        return self.data

    def __add__(self, other: Any) -> Self:
        if isinstance(other, self.__class__):
            self.data += other.data
            self.esc_len += other.esc_len
            return self
        if isinstance(other, str):
            self.data += other
            return self
        self.data = str(self.data + other)
        return self

    def __radd__(self, other: Any) -> Self:
        if isinstance(other, str):
            self.data = other + self.data
            return self
        self.data = str(other + self.data)
        return self

    def __mul__(self, n: int) -> Self:
        self.data *= n
        self.esc_len *= n if n > 0 else 0
        return self

    def __rmul__(self, n: int) -> Self:
        return self.__mul__(n)

    def __len__(self) -> int:
        return self.data.__len__() - self.esc_len

    def len(self) -> int:
        return self.__len__()

    def center(self, width: int) -> Self:
        self.data = self.data.center(width + self.esc_len)
        return self

    def ljust(self, width: int) -> Self:
        self.data = self.data.ljust(width + self.esc_len)
        return self

    def rjust(self, width: int) -> Self:
        self.data = self.data.rjust(width + self.esc_len)
        return self

    def set_bold(self) -> Self:
        bold = self.__from_escseq("style", "bold")
        unset = self.__from_escseq("reset", "bold/dim")
        self.data = bold.data + self.data + unset.data
        self.esc_len += bold.esc_len + unset.esc_len
        return self

    def set_dim(self) -> Self:
        dim = self.__from_escseq("style", "dim")
        unset = self.__from_escseq("reset", "bold/dim")
        self.data = dim.data + self.data + unset.data
        self.esc_len += dim.esc_len + unset.esc_len
        return self

    def set_bold_dim(self) -> Self:
        bold = self.__from_escseq("style", "bold")
        dim = self.__from_escseq("style", "dim")
        unset = self.__from_escseq("reset", "bold/dim")
        self.data = bold.data + dim.data + self.data + unset.data
        self.esc_len += bold.esc_len + dim.esc_len + unset.esc_len
        return self

    def set_italic(self) -> Self:
        italic = self.__from_escseq("style", "italic")
        unset = self.__from_escseq("reset", "italic")
        self.data = italic.data + self.data + unset.data
        self.esc_len += italic.esc_len + unset.esc_len
        return self

    def set_underline(self) -> Self:
        underline = self.__from_escseq("style", "underline")
        unset = self.__from_escseq("reset", "underline")
        self.data = underline.data + self.data + unset.data
        self.esc_len += underline.esc_len + unset.esc_len
        return self

    def set_strikethrough(self) -> Self:
        strikethrough = self.__from_escseq("style", "strikethrough")
        unset = self.__from_escseq("reset", "strikethrough")
        self.data = strikethrough.data + self.data + unset.data
        self.esc_len += strikethrough.esc_len + unset.esc_len
        return self

    def set_color(self, color: str) -> Self:
        color_seq = self.from_escseq("foreground", color)
        if color_seq is None:
            raise ValueError("invalid color")
        unset = self.__from_escseq("reset", "foreground")
        self.data = color_seq.data + self.data + unset.data
        self.esc_len += color_seq.esc_len + unset.esc_len
        return self

    def set_color_bg(self, color: str) -> Self:
        color_seq = self.from_escseq("background", color)
        if color_seq is None:
            raise ValueError("invalid color")
        unset = self.__from_escseq("reset", "background")
        self.data = color_seq.data + self.data + unset.data
        self.esc_len += color_seq.esc_len + unset.esc_len
        return self
