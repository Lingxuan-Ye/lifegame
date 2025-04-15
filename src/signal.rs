use crate::error::QuitOnError;
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use std::sync::atomic::{AtomicBool, AtomicI8, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard, Once};
use std::thread;

pub static TIME_SCALE: TimeScale = TimeScale::new();
pub static RESET: AtomicBool = AtomicBool::new(false);
pub static PAUSE: Pause = Pause::new();
pub static QUIT: AtomicBool = AtomicBool::new(false);

static LISTENER: Once = Once::new();

pub fn setup_listener() {
    let handler = || {
        loop {
            let event = read().quit_on_error();

            let Event::Key(key_event) = event else {
                continue;
            };
            if !key_event.is_press() {
                continue;
            }
            let KeyCode::Char(key) = key_event.code else {
                continue;
            };

            match key.to_ascii_lowercase() {
                'j' => {
                    TIME_SCALE.increment();
                }
                'k' => {
                    TIME_SCALE.decrement();
                }
                'r' => {
                    RESET.store(true, Ordering::Relaxed);
                }
                'p' => {
                    PAUSE.toggle();
                }
                'q' => {
                    PAUSE.unpause();
                    QUIT.store(true, Ordering::Relaxed);
                    break;
                }
                'c' if key_event.modifiers == KeyModifiers::CONTROL => {
                    PAUSE.unpause();
                    QUIT.store(true, Ordering::Relaxed);
                    break;
                }
                _ => (),
            }
        }
    };

    LISTENER.call_once(|| {
        thread::spawn(handler);
    })
}

#[derive(Debug)]
pub struct TimeScale {
    exponent: AtomicI8,
}

impl TimeScale {
    pub const MAX_EXPONENT: i8 = 10;
    pub const MIN_EXPONENT: i8 = -10;

    pub const fn new() -> Self {
        let exponent = AtomicI8::new(0);
        Self { exponent }
    }

    pub fn scale(&self) -> f64 {
        let exponent = self.exponent.load(Ordering::Relaxed) as f64;
        exponent.exp2()
    }

    pub fn increment(&self) -> &Self {
        if self.exponent.load(Ordering::Relaxed) < Self::MAX_EXPONENT {
            self.exponent.fetch_add(1, Ordering::Relaxed);
        }
        self
    }

    pub fn decrement(&self) -> &Self {
        if self.exponent.load(Ordering::Relaxed) > Self::MIN_EXPONENT {
            self.exponent.fetch_sub(1, Ordering::Relaxed);
        }
        self
    }
}

#[derive(Debug)]
pub struct Pause {
    state: Mutex<bool>,
    cvar: Condvar,
}

impl Pause {
    pub const fn new() -> Self {
        let state = Mutex::new(false);
        let cvar = Condvar::new();
        Self { state, cvar }
    }

    pub fn pause(&self) -> &Self {
        *self.state() = true;
        self
    }

    pub fn unpause(&self) -> &Self {
        *self.state() = false;
        self.cvar.notify_all();
        self
    }

    pub fn toggle(&self) -> &Self {
        let mut state = self.state();
        if *state {
            *state = false;
            self.cvar.notify_all();
        } else {
            *state = true;
        }
        self
    }

    pub fn wait_if_paused(&self) -> &Self {
        let mut state = self.state();
        while *state {
            state = match self.cvar.wait(state) {
                Err(_) => unreachable!(),
                Ok(guard) => guard,
            };
        }
        self
    }

    fn state(&self) -> MutexGuard<bool> {
        match self.state.lock() {
            Err(_) => unreachable!(),
            Ok(guard) => guard,
        }
    }
}
