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

    pub fn elapsed(&self) -> Duration {
        self.timezero.elapsed()
    }

    pub fn reset(&mut self) -> &mut Self {
        self.timezero = Instant::now();
        self
    }

    pub fn pause(&mut self) -> PausedTimer<'_> {
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
    pub fn elapsed(&self) -> Duration {
        self.timezero.elapsed()
    }
}

impl Drop for PausedTimer<'_> {
    fn drop(&mut self) {
        self.timer.timezero += self.timezero.elapsed();
    }
}

pub fn fmt_duration(duration: Duration) -> String {
    let mut nanos = duration.as_nanos();

    let secs = nanos / NANOS_PER_SEC;
    nanos %= NANOS_PER_SEC;

    let millis = nanos / NANOS_PER_MILLI;
    nanos %= NANOS_PER_MILLI;

    let micros = nanos / NANOS_PER_MICRO;
    nanos %= NANOS_PER_MICRO;

    format!("{secs} s {millis:>03} ms {micros:>03} μs {nanos:>03} ns")
}
