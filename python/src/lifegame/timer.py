import time
from collections import deque
from typing import Self

from .term import TermString, set_dim


class Timer:

    NS_PER_S = 10**9
    NS_PER_MS = 10**6
    NS_PER_MUS = 10**3

    FMT_SEP = ' - '

    def __init__(self, records_capacity: int = 100_000) -> None:
        self.__timezero = time.time_ns()
        self.__records: deque[int] = deque(maxlen=records_capacity)

    @property
    def records(self) -> deque[int]:
        return self.__records

    def check(self, record: bool = False) -> int:
        elapsed = time.time_ns() - self.__timezero
        if record:
            self.__records.append(elapsed)
        return elapsed

    @staticmethod
    def _style(value: int, unit: str) -> TermString:
        return TermString(f'{value:>3} {set_dim(unit)}', esc_len=9)

    @classmethod
    def format(cls, ns: int) -> TermString:
        s, ns = divmod(ns, cls.NS_PER_S)
        ms, ns = divmod(ns, cls.NS_PER_MS)
        mus, ns = divmod(ns, cls.NS_PER_MUS)
        return (
            cls._style(s, 's') + cls.FMT_SEP +
            cls._style(ms, 'ms') + cls.FMT_SEP +
            cls._style(mus, 'μs') + cls.FMT_SEP +
            cls._style(ns, 'ns')
        )

    def check_fmt(self, record: bool = False) -> TermString:
        return self.format(self.check(record))

    def check_delta(self, record: bool = False) -> int:
        elapsed = time.time_ns() - self.__timezero
        if self.__records:
            delta = elapsed - self.__records[-1]
        else:
            delta = elapsed
        if record:
            self.__records.append(elapsed)
        return delta

    def reset(self) -> Self:
        self.__timezero = time.time_ns()
        self.__records.clear()
        return self
