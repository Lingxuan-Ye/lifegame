use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Timer {
    global_start: Instant,
    frame_start: Instant,
    last_frame: Duration,
}

impl Timer {
    pub fn start() -> Self {
        Self {
            global_start: Instant::now(),
            frame_start: Instant::now(),
            last_frame: Duration::default(),
        }
    }

    pub fn global(&self) -> Duration {
        self.global_start.elapsed()
    }

    pub fn frame(&self) -> Duration {
        self.frame_start.elapsed()
    }

    pub fn last_frame(&self) -> Duration {
        self.last_frame
    }

    pub fn tick(&mut self) -> &mut Self {
        self.last_frame = self.frame();
        self.frame_start = Instant::now();
        self
    }

    pub fn pause(&mut self) -> PausedTimer<'_> {
        PausedTimer {
            start: Instant::now(),
            timer: self,
        }
    }
}

#[derive(Debug)]
pub struct PausedTimer<'a> {
    start: Instant,
    timer: &'a mut Timer,
}

impl PausedTimer<'_> {
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

impl Drop for PausedTimer<'_> {
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        self.timer.frame_start += elapsed;
    }
}

pub fn fmt_duration(duration: Duration) -> String {
    const NANOS_PER_SEC: u128 = Duration::from_secs(1).as_nanos();
    const NANOS_PER_MILLI: u128 = Duration::from_millis(1).as_nanos();
    const NANOS_PER_MICRO: u128 = Duration::from_micros(1).as_nanos();

    let mut nanos = duration.as_nanos();

    let secs = nanos / NANOS_PER_SEC;
    nanos %= NANOS_PER_SEC;

    let millis = nanos / NANOS_PER_MILLI;
    nanos %= NANOS_PER_MILLI;

    let micros = nanos / NANOS_PER_MICRO;
    nanos %= NANOS_PER_MICRO;

    format!("{secs} s {millis:>03} ms {micros:>03} μs {nanos:>03} ns")
}
