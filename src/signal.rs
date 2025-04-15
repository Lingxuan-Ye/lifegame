use crate::error::QuitOnError;
use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard};
use std::thread;

pub static RESET : AtomicBool = AtomicBool::new(false);
pub static PAUSE: Pause = Pause::new();
pub static QUIT: AtomicBool = AtomicBool::new(false);

pub fn spawn_listener() {
    thread::spawn(|| {
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
                'r' => {
                    RESET.store(true, Ordering::Relaxed);
                },
                'p' => {
                    PAUSE.toggle();
                }
                'q' => {
                    QUIT.store(true, Ordering::Relaxed);
                    break;
                }
                'c' if key_event.modifiers == KeyModifiers::CONTROL => {
                    QUIT.store(true, Ordering::Relaxed);
                    break;
                }
                _ => (),
            }
        }
    });
}

#[derive(Debug)]
pub struct Pause {
    state: Mutex<bool>,
    cvar: Condvar,
}

impl Pause {
    pub const fn new() -> Self {
        Self {
            state: Mutex::new(false),
            cvar: Condvar::new(),
        }
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
