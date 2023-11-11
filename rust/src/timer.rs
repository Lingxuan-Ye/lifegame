use crate::term::{utils::set_dim, TermString};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

struct LimitedDeque<T> {
    data: VecDeque<T>,
    max_len: usize,
}

impl<T> LimitedDeque<T> {
    fn new(max_len: usize) -> Self {
        LimitedDeque {
            data: VecDeque::new(),
            max_len,
        }
    }
    fn back(&self) -> Option<&T> {
        self.data.back()
    }
    fn clear(&mut self) {
        self.data.clear()
    }
    fn push_back(&mut self, value: T) -> Option<T> {
        self.data.push_back(value);
        if self.data.len() > self.max_len {
            self.data.pop_front()
        } else {
            None
        }
    }
}

pub struct Timer {
    timezero: Instant,
    records: LimitedDeque<u128>,
}

impl Timer {
    const NANOS_PER_SEC: u128 = Duration::from_secs(1).as_nanos();
    const NANOS_PER_MILLI: u128 = Duration::from_millis(1).as_nanos();
    const NANOS_PER_MICRO: u128 = Duration::from_micros(1).as_nanos();

    const FMT_SEP: &str = " - ";

    pub fn new() -> Self {
        Self {
            timezero: Instant::now(),
            records: LimitedDeque::new(100_000),
        }
    }

    pub fn records(&self) -> &VecDeque<u128> {
        &self.records.data
    }

    pub fn check(&mut self, record: bool) -> u128 {
        let elapsed = self.timezero.elapsed().as_nanos();
        if record {
            self.records.push_back(elapsed);
        }
        elapsed
    }

    pub fn check_fmt(&mut self, record: bool) -> TermString {
        let mut nanos = self.check(record);

        let secs = nanos / Self::NANOS_PER_SEC;
        nanos %= Self::NANOS_PER_SEC;

        let millis = nanos / Self::NANOS_PER_MILLI;
        nanos %= Self::NANOS_PER_MILLI;

        let micros = nanos / Self::NANOS_PER_MICRO;
        nanos %= Self::NANOS_PER_MICRO;

        format!("{secs:>3} ")
            + set_dim("s")
            + Self::FMT_SEP
            + format!("{millis:>3} ")
            + set_dim("ms")
            + Self::FMT_SEP
            + format!("{micros:>3} ")
            + set_dim("μs")
            + Self::FMT_SEP
            + format!("{nanos:>3} ")
            + set_dim("ns")
            + Self::FMT_SEP
    }

    pub fn check_delta(&mut self, record: bool) -> u128 {
        let elapsed = self.check(record);
        if let Some(&latest) = self.records.back() {
            elapsed - latest
        } else {
            elapsed
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.timezero = Instant::now();
        self.records.clear();
        self
    }
}
