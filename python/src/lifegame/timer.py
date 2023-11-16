import time
from collections import deque
from typing import Self

from .term import TermString


class Timer:
    NANOS_PER_SEC = 10**9
    NANOS_PER_MILLI = 10**6
    NANOS_PER_MICRO = 10**3

    FMT_SEP = " - "

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
    def _measurement_fmt(value: int, unit: str) -> TermString:
        return f"{value:>3} " + TermString(unit).set_dim()

    @classmethod
    def format(cls, nanos: int) -> TermString:
        secs, nanos = divmod(nanos, cls.NANOS_PER_SEC)
        millis, nanos = divmod(nanos, cls.NANOS_PER_MILLI)
        micros, nanos = divmod(nanos, cls.NANOS_PER_MICRO)
        return (
            cls._measurement_fmt(secs, "s")
            + cls.FMT_SEP
            + cls._measurement_fmt(millis, "ms")
            + cls.FMT_SEP
            + cls._measurement_fmt(micros, "μs")
            + cls.FMT_SEP
            + cls._measurement_fmt(nanos, "ns")
        )

    def check_fmt(self, record: bool = False) -> TermString:
        return self.format(self.check(record))

    def check_delta(self, record: bool = False) -> int:
        elapsed = self.check(record)
        if self.__records:
            delta = elapsed - self.__records[-1]
        else:
            delta = elapsed
        return delta

    def reset(self) -> Self:
        self.__timezero = time.time_ns()
        self.__records.clear()
        return self
