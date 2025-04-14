use crossterm::event::{Event, KeyCode, KeyModifiers, read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub static PAUSE: AtomicBool = AtomicBool::new(false);
pub static QUIT: AtomicBool = AtomicBool::new(false);

pub fn spawn_listener() {
    thread::spawn(|| {
        loop {
            match read() {
                Err(_error) => unimplemented!(), // TODO
                Ok(event) => {
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
                        'p' => {
                            PAUSE.fetch_not(Ordering::Relaxed);
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
            }
        }
    });
}
