import re
import time

from lifegame.timer import Timer


def test_check():
    timer = Timer()
    time.sleep(0.01)
    assert 0.01e9 <= timer.check() <= 0.011e9


def test_format():
    formatted = Timer.format(1_234_567_890)
    assert re.search(
        r'1.*s.*234.*ms.*567.*μs.*890.*ns', formatted.to_str(), re.DOTALL
    )
