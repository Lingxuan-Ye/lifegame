use crate::term::{TermString, ToTermString};
use std::cell::RefCell;
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
    records: RefCell<LimitedDeque<u128>>,
}

impl Timer {
    pub const NANOS_PER_SEC: u128 = Duration::from_secs(1).as_nanos();
    pub const NANOS_PER_MILLI: u128 = Duration::from_millis(1).as_nanos();
    pub const NANOS_PER_MICRO: u128 = Duration::from_micros(1).as_nanos();

    const FMT_SEP: &str = " - ";

    pub fn new() -> Self {
        Self {
            timezero: Instant::now(),
            records: RefCell::new(LimitedDeque::new(100_000)),
        }
    }

    pub fn records(&self) -> VecDeque<u128> {
        self.records.borrow().data.clone()
    }

    pub fn check(&self, record: bool) -> u128 {
        let elapsed = self.timezero.elapsed().as_nanos();
        if record {
            self.records.borrow_mut().push_back(elapsed);
        }
        elapsed
    }

    pub fn check_fmt(&self, record: bool) -> TermString {
        let mut nanos = self.check(record);

        let secs = nanos / Self::NANOS_PER_SEC;
        nanos %= Self::NANOS_PER_SEC;

        let millis = nanos / Self::NANOS_PER_MILLI;
        nanos %= Self::NANOS_PER_MILLI;

        let micros = nanos / Self::NANOS_PER_MICRO;
        nanos %= Self::NANOS_PER_MICRO;

        secs.to_tstr().rjust(3)
            + "s".to_tstr().set_dim()
            + Self::FMT_SEP
            + millis.to_tstr().rjust(3)
            + "ms".to_tstr().set_dim()
            + Self::FMT_SEP
            + micros.to_tstr().rjust(3)
            + "μs".to_tstr().set_dim()
            + Self::FMT_SEP
            + nanos.to_tstr().rjust(3)
            + "ns".to_tstr().set_dim()
    }

    pub fn check_delta(&self, record: bool) -> u128 {
        let elapsed = self.check(record);
        if let Some(&latest) = self.records.borrow_mut().back() {
            elapsed - latest
        } else {
            elapsed
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.timezero = Instant::now();
        self.records.borrow_mut().clear();
        self
    }
}
