use crate::biosquare::{BioSquare, Cell};
use crate::filter::Filter;
use crate::signal;
use anyhow::Result;
use crossterm::style::Stylize;
use crossterm::{QueueableCommand, cursor, style, terminal};
use eoe::QuitOnError;
use matreex::Matrix;
use rand::rngs::ThreadRng;
use std::io::Write;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Screen<F, O>
where
    F: Filter,
    O: Write,
{
    biosquare: BioSquare,
    genesis: Matrix<Cell>,
    fps_max: f64,
    show_stats: bool,
    timer: Timer,
    rng: ThreadRng,
    filter: F,
    output: O,
}

impl<F, O> Screen<F, O>
where
    F: Filter,
    O: Write,
{
    pub fn new(
        genesis: Matrix<Cell>,
        fps_max: f64,
        show_stats: bool,
        filter: F,
        output: O,
    ) -> Result<Self> {
        let biosquare = BioSquare::new(genesis.clone());
        let fps_max = if fps_max >= 0.0 { fps_max } else { 60.0 };
        let timer = Timer::start();
        let rng = rand::rng();

        let mut tui = Self {
            biosquare,
            genesis,
            fps_max,
            show_stats,
            timer,
            rng,
            filter,
            output,
        };

        tui.enter_alternate_screen()?;

        Ok(tui)
    }

    pub fn run(&mut self) -> Result<()> {
        'outer: loop {
            self.timer.tick();

            self.wait_if_paused();

            if signal::QUIT.get() {
                break Ok(());
            }

            if signal::FLIP.take() {
                self.random_flip();
            }

            if signal::RESET.take() {
                self.reset();
            }

            if let Err(error) = self.render() {
                break Err(error);
            }

            self.biosquare.evolve();

            while self.timer.frame().as_secs_f64() < self.frame_duration_min() {
                if signal::QUIT.get() {
                    break 'outer Ok(());
                }
            }
        }
    }

    fn render(&mut self) -> Result<()> {
        self.output
            .queue(terminal::BeginSynchronizedUpdate)?
            .queue(cursor::MoveTo(0, 0))?;

        let matrix = self.biosquare.observe();

        for row in matrix.iter_rows() {
            for cell in row {
                let view = self.filter.filter(*cell);
                self.output.queue(style::Print(view))?;
            }
            self.output.queue(cursor::MoveToNextLine(1))?;
        }

        if self.show_stats {
            self.render_stats()?;
        }

        self.output
            .queue(terminal::EndSynchronizedUpdate)?
            .flush()?;

        Ok(())
    }

    fn render_stats(&mut self) -> Result<()> {
        self.output.queue(cursor::MoveToNextLine(2))?;

        let generation = self.biosquare.generation();
        let population = self.biosquare.population();
        let density = self.biosquare.density();
        let fps = 1.0 / self.timer.last_frame().as_secs_f64();
        let runtime = self.timer.global();

        self.render_measurement("Generation", format!("{generation}"))?
            .render_measurement("Population", format!("{population}"))?
            .render_measurement("Density", format!("{:.2} %", density * 100.0))?
            .render_measurement("FPS", format!("{fps:.2}"))?
            .render_measurement("Runtime", fmt_duration(runtime))?;

        Ok(())
    }

    /// # Notes
    ///
    /// `value` is a `String` rather than a generic `T: Display` because not
    /// all `T`s support alignment. However, this introduces a small overhead
    /// due to formatting it twice.
    ///
    /// Additionally, `key` and `value` should avoid containing full-width or
    /// non-printable characters, or the alignment will be incorrect.
    fn render_measurement(&mut self, key: &str, value: String) -> Result<&mut Self> {
        const KEY_WIDTH: usize = 20;
        const VALUE_WIDTH: usize = 40;

        let key = format!("{key:<KEY_WIDTH$}");
        let value = format!("{value:>VALUE_WIDTH$}");

        self.output
            .queue(style::Print(key.bold()))?
            .queue(style::Print(value))?
            .queue(cursor::MoveToNextLine(1))?;

        Ok(self)
    }

    fn wait_if_paused(&mut self) {
        let _paused = self.timer.pause();
        signal::PAUSE.wait_if_paused();
    }

    fn random_flip(&mut self) {
        self.biosquare.random_flip(&mut self.rng);
    }

    fn reset(&mut self) {
        self.biosquare = BioSquare::new(self.genesis.clone());
        self.timer = Timer::start();
    }

    fn frame_duration_min(&self) -> f64 {
        signal::TIME_SCALE.scale() / self.fps_max
    }

    fn enter_alternate_screen(&mut self) -> Result<()> {
        self.output
            .queue(terminal::EnterAlternateScreen)?
            .queue(terminal::DisableLineWrap)?
            .queue(cursor::Hide)?
            .flush()?;

        terminal::enable_raw_mode()?;

        Ok(())
    }

    fn leave_alternate_screen(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;

        self.output
            .queue(cursor::Show)?
            .queue(terminal::EnableLineWrap)?
            .queue(terminal::LeaveAlternateScreen)?
            .flush()?;

        Ok(())
    }
}

impl<F, O> Drop for Screen<F, O>
where
    F: Filter,
    O: Write,
{
    fn drop(&mut self) {
        self.leave_alternate_screen().quit_on_error();
    }
}

#[derive(Debug)]
struct Timer {
    global_start: Instant,
    frame_start: Instant,
    last_frame: Duration,
}

impl Timer {
    fn start() -> Self {
        Self {
            global_start: Instant::now(),
            frame_start: Instant::now(),
            last_frame: Duration::default(),
        }
    }

    fn global(&self) -> Duration {
        self.global_start.elapsed()
    }

    fn frame(&self) -> Duration {
        self.frame_start.elapsed()
    }

    fn last_frame(&self) -> Duration {
        self.last_frame
    }

    fn tick(&mut self) {
        self.last_frame = self.frame();
        self.frame_start = Instant::now();
    }

    fn pause(&mut self) -> PausedTimer<'_> {
        PausedTimer {
            start: Instant::now(),
            timer: self,
        }
    }
}

#[derive(Debug)]
struct PausedTimer<'a> {
    start: Instant,
    timer: &'a mut Timer,
}

impl Drop for PausedTimer<'_> {
    fn drop(&mut self) {
        self.timer.frame_start += self.start.elapsed();
    }
}

fn fmt_duration(duration: Duration) -> String {
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
