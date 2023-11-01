from collections import UserString
from typing import Any, Self

ERASE = {
    'screen': '\u001b[2J',
}
RESET = {
    'all': '\u001b[0m',
    'bold/dim': '\u001b[22m',
    'italic': '\u001b[23m',
    'underline': '\u001b[24m',
    'blink': '\u001b[25m',
    'reverse': '\u001b[27m',
    'invisible ': '\u001b[28m',
    'strikethrough': '\u001b[29m',
    'foreground': '\u001b[39m',
    'background': '\u001b[49m',
    'cursor': '\u001b[H',
}
STYLE = {
    'bold': '\u001b[1m',
    'dim': '\u001b[2m',
    'italic': '\u001b[3m',
    'underline': '\u001b[4m',
    'blink': '\u001b[5m',
    'reverse': '\u001b[7m',
    'invisible ': '\u001b[8m',
    'strikethrough': '\u001b[9m',
}
FOREGROUND = {
    'black': '\u001b[30m',
    'red': '\u001b[31m',
    'green': '\u001b[32m',
    'yellow': '\u001b[33m',
    'blue': '\u001b[34m',
    'magenta': '\u001b[35m',
    'cyan': '\u001b[36m',
    'white': '\u001b[37m',
}
BACKGROUND = {
    'black': '\u001b[40m',
    'red': '\u001b[41m',
    'green': '\u001b[42m',
    'yellow': '\u001b[43m',
    'blue': '\u001b[44m',
    'magenta': '\u001b[45m',
    'cyan': '\u001b[46m',
    'white': '\u001b[47m',
}


def erase_screen() -> None:
    print(ERASE['screen'] + RESET['cursor'], end='')


def reset_cursor() -> None:
    print(RESET['cursor'], end='')


def set_bold(value: str) -> str:
    return STYLE['bold'] + value + RESET['bold/dim']


def set_dim(value: str) -> str:
    return STYLE['dim'] + value + RESET['bold/dim']


def set_bold_dim(value: str) -> str:
    return STYLE['bold'] + STYLE['dim'] + value + RESET['bold/dim']


def set_color(value: str, color: str) -> str:
    return FOREGROUND[color] + value + RESET['foreground']


def set_color_bg(value: str, color: str) -> str:
    return BACKGROUND[color] + value + RESET['background']


class TermString(UserString):
    """
    Inherited methods that return instances of this class (`Self`)
    and are not explicitly overridden may have UNPREDICTABLE behavior.
    It is strongly advised against calling them.

    Omit `esc_len` on initialization if `seq` is instance of this class.
    """

    esc_len: int

    def __init__(self, seq: Any, *, esc_len: int = 0) -> None:
        super().__init__(seq)
        if isinstance(seq, self.__class__):
            esc_len += seq.esc_len
        self.esc_len = esc_len

    def __len__(self) -> int:
        return super().__len__() - self.esc_len

    def __add__(self, other: Any) -> Self:
        inst = super().__add__(other)
        esc_len = self.esc_len
        if isinstance(other, self.__class__):
            esc_len += other.esc_len
        inst.esc_len = esc_len
        return inst

    def __mul__(self, n: int) -> Self:
        inst = super().__mul__(n)
        inst.esc_len = self.esc_len * n if n > 0 else 0
        return inst

    def center(self, width: int, *args: Any) -> Self:
        inst = super().center(width + self.esc_len, *args)
        inst.esc_len = self.esc_len
        return inst

    def ljust(self, width: int, *args: Any) -> Self:
        inst = super().ljust(width + self.esc_len, *args)
        inst.esc_len = self.esc_len
        return inst

    def rjust(self, width: int, *args: Any) -> Self:
        inst = super().rjust(width + self.esc_len, *args)
        inst.esc_len = self.esc_len
        return inst

    def to_str(self) -> str:
        return str(self)
