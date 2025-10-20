use crate::biosquare::{BioSquare, Cell};
use crate::bounded::Bounded;
use crate::filter::Filter;
use crate::signal;
use anyhow::Result;
use crossterm::style::Stylize;
use crossterm::{QueueableCommand, cursor, style, terminal};
use matreex::Matrix;
use std::io::Write;
use std::ops::RangeInclusive;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Tui<F, O>
where
    F: Filter,
    O: Write,
{
    genesis: Matrix<Cell>,
    biosquare: BioSquare,
    fps_max: FpsMax,
    show_stats: bool,
    filter: F,
    output: O,
    timer: Timer,
}

impl<F, O> Tui<F, O>
where
    F: Filter,
    O: Write,
{
    pub fn new(
        genesis: Matrix<Cell>,
        fps_max: FpsMax,
        show_stats: bool,
        filter: F,
        output: O,
    ) -> Self {
        let biosquare = BioSquare::new(genesis.clone());
        let timer = Timer::start();

        Self {
            genesis,
            biosquare,
            fps_max,
            show_stats,
            filter,
            output,
            timer,
        }
    }

    pub fn run(&mut self) -> Result<&mut Self> {
        self.enter_alternate_screen()?;

        let result = 'outer: loop {
            self.timer.tick();

            if signal::RESET.take() {
                self.reset();
            }

            self.wait_if_paused();

            if signal::QUIT.get() {
                break Ok(());
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
        };

        self.leave_alternate_screen()?;

        result?;

        Ok(self)
    }

    fn render(&mut self) -> Result<&mut Self> {
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

        Ok(self)
    }

    fn render_stats(&mut self) -> Result<&mut Self> {
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

        Ok(self)
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

    fn reset(&mut self) -> &mut Self {
        self.biosquare = BioSquare::new(self.genesis.clone());
        self.timer = Timer::start();
        self
    }

    fn wait_if_paused(&mut self) -> &mut Self {
        let timer = self.timer.pause();
        signal::PAUSE.wait_if_paused();
        drop(timer);
        self
    }

    fn frame_duration_min(&self) -> f64 {
        signal::TIME_SCALE.scale() / self.fps_max.get()
    }

    fn enter_alternate_screen(&mut self) -> Result<&mut Self> {
        self.output
            .queue(terminal::SetTitle("Lifegame"))?
            .queue(cursor::Hide)?
            .queue(terminal::EnterAlternateScreen)?
            .queue(terminal::DisableLineWrap)?
            .queue(terminal::Clear(terminal::ClearType::All))?
            .flush()?;

        terminal::enable_raw_mode()?;

        Ok(self)
    }

    fn leave_alternate_screen(&mut self) -> Result<&mut Self> {
        terminal::disable_raw_mode()?;

        self.output
            .queue(terminal::EnableLineWrap)?
            .queue(terminal::LeaveAlternateScreen)?
            .queue(cursor::Show)?
            .flush()?;

        Ok(self)
    }
}

#[derive(Debug)]
pub struct FpsMax(f64);

impl Bounded<f64, RangeInclusive<f64>> for FpsMax {
    const RANGE: RangeInclusive<f64> = 0.0..=f64::INFINITY;

    fn new_or_default(value: f64) -> Self {
        if Self::RANGE.contains(&value) {
            Self(value)
        } else {
            Self::default()
        }
    }

    fn get(&self) -> &f64 {
        &self.0
    }
}

impl Default for FpsMax {
    fn default() -> Self {
        Self(60.0)
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

    fn tick(&mut self) -> &mut Self {
        self.last_frame = self.frame();
        self.frame_start = Instant::now();
        self
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

impl PausedTimer<'_> {
    fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

impl Drop for PausedTimer<'_> {
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        self.timer.frame_start += elapsed;
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
