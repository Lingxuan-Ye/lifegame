use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use eoe::QuitOnError;
use std::sync::atomic::{AtomicBool, AtomicI8, Ordering::Relaxed};
use std::sync::{Condvar, Mutex, MutexGuard, Once};
use std::thread;

pub static TIME_SCALE: TimeScale = TimeScale::new();
pub static PAUSE: Pause = Pause::new();
pub static FLIP: Flip = Flip::new();
pub static RESET: Reset = Reset::new();
pub static QUIT: Quit = Quit::new();

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
                'p' => {
                    PAUSE.toggle();
                }
                'f' => {
                    FLIP.set();
                }
                'r' => {
                    RESET.set();
                }
                'q' => {
                    PAUSE.unset();
                    QUIT.set();
                    break;
                }
                'c' if key_event.modifiers == KeyModifiers::CONTROL => {
                    PAUSE.unset();
                    QUIT.set();
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

    const fn new() -> Self {
        let exponent = AtomicI8::new(0);
        Self { exponent }
    }

    fn increment(&self) {
        let _ = self.exponent.fetch_update(Relaxed, Relaxed, |exponent| {
            if exponent < Self::MAX_EXPONENT {
                Some(exponent + 1)
            } else {
                None
            }
        });
    }

    fn decrement(&self) {
        let _ = self.exponent.fetch_update(Relaxed, Relaxed, |exponent| {
            if exponent > Self::MIN_EXPONENT {
                Some(exponent - 1)
            } else {
                None
            }
        });
    }

    pub fn scale(&self) -> f64 {
        let exponent = self.exponent.load(Relaxed) as f64;
        exponent.exp2()
    }
}

#[derive(Debug)]
pub struct Pause {
    state: Mutex<bool>,
    cvar: Condvar,
}

impl Pause {
    const fn new() -> Self {
        let state = Mutex::new(false);
        let cvar = Condvar::new();
        Self { state, cvar }
    }

    fn state(&self) -> MutexGuard<'_, bool> {
        match self.state.lock() {
            Err(_) => unreachable!(),
            Ok(guard) => guard,
        }
    }

    fn toggle(&self) {
        let mut state = self.state();
        if *state {
            *state = false;
            self.cvar.notify_all();
        } else {
            *state = true;
        }
    }

    fn unset(&self) {
        *self.state() = false;
        self.cvar.notify_all();
    }

    pub fn wait_if_paused(&self) {
        let mut state = self.state();
        while *state {
            state = match self.cvar.wait(state) {
                Err(_) => unreachable!(),
                Ok(guard) => guard,
            };
        }
    }
}

#[derive(Debug)]
pub struct Flip {
    state: AtomicBool,
}

impl Flip {
    const fn new() -> Self {
        let state = AtomicBool::new(false);
        Self { state }
    }

    fn set(&self) {
        self.state.store(true, Relaxed);
    }

    pub fn take(&self) -> bool {
        self.state.swap(false, Relaxed)
    }
}

#[derive(Debug)]
pub struct Reset {
    state: AtomicBool,
}

impl Reset {
    const fn new() -> Self {
        let state = AtomicBool::new(false);
        Self { state }
    }

    fn set(&self) {
        self.state.store(true, Relaxed);
    }

    pub fn take(&self) -> bool {
        self.state.swap(false, Relaxed)
    }
}

#[derive(Debug)]
pub struct Quit {
    state: AtomicBool,
}

impl Quit {
    const fn new() -> Self {
        let state = AtomicBool::new(false);
        Self { state }
    }

    fn set(&self) {
        self.state.store(true, Relaxed);
    }

    pub fn get(&self) -> bool {
        self.state.load(Relaxed)
    }
}
