use crate::style::StylizeExt;
use std::fmt;
use std::time::{Duration, Instant};

const NANOS_PER_SEC: u128 = Duration::from_secs(1).as_nanos();
const NANOS_PER_MILLI: u128 = Duration::from_millis(1).as_nanos();
const NANOS_PER_MICRO: u128 = Duration::from_micros(1).as_nanos();

#[derive(Clone, Debug)]
pub struct Timer {
    timezero: Instant,
}

impl Timer {
    pub fn start() -> Self {
        let timezero = Instant::now();
        Self { timezero }
    }

    pub fn elapsed(&self) -> Elapsed {
        Elapsed(self.timezero.elapsed())
    }

    pub fn reset(&mut self) -> &mut Self {
        self.timezero = Instant::now();
        self
    }

    pub fn pause(&mut self) -> PausedTimer {
        let timezero = Instant::now();
        let timer = self;
        PausedTimer { timezero, timer }
    }
}

#[derive(Debug)]
pub struct PausedTimer<'a> {
    timezero: Instant,
    timer: &'a mut Timer,
}

impl PausedTimer<'_> {
    pub fn elapsed(&self) -> Elapsed {
        Elapsed(self.timezero.elapsed())
    }
}

impl Drop for PausedTimer<'_> {
    fn drop(&mut self) {
        self.timer.timezero += self.timezero.elapsed();
    }
}

#[derive(Clone, Debug)]
pub struct Elapsed(Duration);

impl Elapsed {
    pub fn duration(&self) -> Duration {
        self.0
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nanos = self.duration().as_nanos();

        let secs = nanos / NANOS_PER_SEC;
        nanos %= NANOS_PER_SEC;

        let millis = nanos / NANOS_PER_MILLI;
        nanos %= NANOS_PER_MILLI;

        let micros = nanos / NANOS_PER_MICRO;
        nanos %= NANOS_PER_MICRO;

        write!(
            f,
            "{secs:>} {} {millis:>03} {} {micros:>03} {} {nanos:>03} {}",
            "s".unit(),
            "ms".unit(),
            "μs".unit(),
            "ns".unit(),
        )
    }
}
