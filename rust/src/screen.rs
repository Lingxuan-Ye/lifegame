pub mod config;

use crate::biosquare::BioSquare;
use crate::term::{erase_screen, reset_cursor};
use crate::term::{TermString, ToTermString};
use crate::timer::Timer;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub type Rows<'a> = Box<dyn Iterator<Item = String> + 'a>;

struct Style {
    x_offset: usize,
    y_offset: usize,
    section_sep: usize,
    label_width: usize,
    value_width: usize,
}

pub struct Screen {
    biosquare: BioSquare,
    timer: Timer,
    iterno: usize,
    iterno_max: Option<usize>,
    fps_max: f64,
    show_stats: bool,
    style: Style,
}

impl Screen {
    pub fn new(biosquare: BioSquare) -> Self {
        Self {
            biosquare,
            timer: Timer::new(),
            iterno: 0,
            iterno_max: None,
            fps_max: f64::INFINITY,
            show_stats: true,
            style: Style {
                x_offset: 2,
                y_offset: 1,
                section_sep: 2,
                label_width: 20,
                value_width: 40,
            },
        }
    }

    fn seperate_sections(&self) -> Rows {
        Box::new((0..self.style.section_sep).map(|_| String::from("")))
    }

    fn measurement_fmt(&self, label: TermString, value: TermString) -> TermString {
        let label_fmt = label.set_bold().ljust(self.style.label_width);
        let value_fmt = value.rjust(self.style.value_width);
        label_fmt + value_fmt
    }

    pub fn observe(&self) -> Rows {
        let density = self.biosquare.population_density();
        let density_fmt = format!("{:.2} %", density * 100.0).to_tstr();

        let fps = Timer::NANOS_PER_SEC as f64 / self.timer.check_delta(false) as f64;
        let fps_fmt = format!("{:.2}", fps).to_tstr();

        Box::new(
            [
                ("Iteration".to_tstr(), self.iterno.to_tstr()),
                ("Population Density".to_tstr(), density_fmt),
                ("FPS".to_tstr(), fps_fmt),
                ("Runtime".to_tstr(), self.timer.check_fmt(true)),
            ]
            .into_iter()
            .map(|(label, value)| self.measurement_fmt(label, value).to_string()),
        )
    }

    fn exit_message(&self) -> Rows {
        let message = "GAME OVER";
        Box::new((0..1).map(move |_| {
            message
                .to_tstr()
                .set_bold()
                .set_color("green")
                .unwrap()
                .to_string()
        }))
    }

    fn render(&self, is_last_frame: bool) -> Rows {
        let mut rows = self.biosquare.observe();
        if self.show_stats {
            rows = Box::new(rows.chain(self.seperate_sections()).chain(self.observe()));
        }
        if is_last_frame {
            rows = Box::new(
                rows.chain(self.seperate_sections())
                    .chain(self.exit_message()),
            );
        }
        rows
    }

    fn offset<'a>(&'a self, frame: Rows<'a>) -> Rows<'a> {
        Box::new(
            (0..self.style.y_offset).map(|_| String::from("")).chain(
                frame.map(|row| String::from(" ").repeat(self.style.x_offset) + row.as_ref()),
            ),
        )
    }

    pub fn display(&self, is_last_frame: bool) {
        reset_cursor();
        for row in self.offset(self.render(is_last_frame)) {
            println!("{row}")
        }
    }

    pub fn play(&mut self) {
        let recv_sigint = Arc::new(AtomicBool::new(false));
        let r = recv_sigint.clone();

        ctrlc::set_handler(move || {
            r.store(true, Ordering::SeqCst);
        })
        .expect("error setting Ctrl-C handler");

        let frame_duration_min: f64 = Timer::NANOS_PER_SEC as f64 / self.fps_max;

        erase_screen();
        self.timer.reset();

        'outer: while !recv_sigint.load(Ordering::SeqCst) {
            if let Some(iterno_max) = self.iterno_max {
                if self.iterno > iterno_max {
                    break;
                }
            }

            let start = self.timer.check(false);
            self.display(false);
            self.biosquare.generate();
            self.iterno += 1;

            while ((self.timer.check(false) - start) as f64) < frame_duration_min {
                if recv_sigint.load(Ordering::SeqCst) {
                    break 'outer;
                }
            }
        }

        self.display(true)
    }
}
