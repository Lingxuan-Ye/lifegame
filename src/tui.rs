use crate::biosquare::BioSquare;
use crate::cell::Cell;
use crate::filter::Filter;
use crate::signal;
use crate::style::StylizeExt;
use crate::timer::{Timer, fmt_duration};
use anyhow::{Result, ensure};
use crossterm::{QueueableCommand, cursor, style, terminal};
use matreex::Matrix;
use std::io::Write;
use std::sync::atomic::Ordering;

#[derive(Clone, Debug)]
pub struct Tui<F, O>
where
    F: Filter,
    O: Write,
{
    genesis: Matrix<Cell>,
    biosquare: BioSquare,
    filter: F,
    output: O,
    show_stats: bool,
    fps_max: f64,
    global_timer: Timer,
    frame_timer: Timer,
}

impl<F, O> Tui<F, O>
where
    F: Filter,
    O: Write,
{
    pub fn new(genesis: Matrix<Cell>, filter: F, output: O) -> Self {
        let biosquare = BioSquare::new(genesis.clone());
        let show_stats = false;
        let fps_max = f64::INFINITY;
        let global_timer = Timer::start();
        let frame_timer = Timer::start();

        Self {
            genesis,
            biosquare,
            filter,
            output,
            show_stats,
            fps_max,
            global_timer,
            frame_timer,
        }
    }

    pub fn show_stats(&mut self) -> &mut Self {
        self.show_stats = true;
        self
    }

    pub fn hide_stats(&mut self) -> &mut Self {
        self.show_stats = false;
        self
    }

    pub fn set_fps_max(&mut self, fps_max: f64) -> Result<&mut Self> {
        ensure!(
            (0.0..=f64::INFINITY).contains(&fps_max),
            "value cannot be NaN or negative",
        );
        self.fps_max = fps_max;
        Ok(self)
    }

    pub fn run(&mut self) -> Result<&mut Self> {
        self.enter_alternate_screen()?;

        let frame_duration = 1.0 / self.fps_max;

        let result = 'outer: loop {
            if let Err(error) = self.render() {
                break Err(error);
            }
            self.frame_timer.reset();
            self.biosquare.evolve();

            if self.should_reset() {
                self.reset();
            }

            self.wait_if_paused();

            while self.frame_timer.elapsed().as_secs_f64() < frame_duration {
                if self.should_quit() {
                    break 'outer Ok(());
                }
            }

            if self.should_quit() {
                break Ok(());
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
        let fps = 1.0 / self.frame_timer.elapsed().as_secs_f64();
        let runtime = self.global_timer.elapsed();

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
    /// due to formatting the value twice.
    ///
    /// Additionally, if `value` contains full-width or non-printable characters,
    /// it may not align correctly.
    fn render_measurement(&mut self, key: &str, value: String) -> Result<&mut Self> {
        const KEY_WIDTH: usize = 20;
        const VALUE_WIDTH: usize = 40;

        let key = format!("{:<KEY_WIDTH$}", key);
        let value = format!("{:>VALUE_WIDTH$}", value);

        self.output
            .queue(style::Print(key.key()))?
            .queue(style::Print(value))?
            .queue(cursor::MoveToNextLine(1))?;

        Ok(self)
    }

    fn should_reset(&mut self) -> bool {
        signal::RESET.load(Ordering::Relaxed)
    }

    fn reset(&mut self) -> &mut Self {
        signal::RESET.store(false, Ordering::Relaxed);
        self.biosquare = BioSquare::new(self.genesis.clone());
        self.global_timer.reset();
        self.frame_timer.reset();
        self
    }

    fn wait_if_paused(&mut self) -> &mut Self {
        let global_timer = self.global_timer.pause();
        let frame_timer = self.frame_timer.pause();
        signal::PAUSE.wait_if_paused();
        drop(global_timer);
        drop(frame_timer);
        self
    }

    fn should_quit(&mut self) -> bool {
        signal::QUIT.load(Ordering::Relaxed)
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
