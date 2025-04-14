use crate::biosquare::BioSquare;
use crate::cell::Cell;
use crate::filter::Filter;
use crate::style::StylizeExt;
use crate::timer::Timer;
use anyhow::Result;
use crossterm::{QueueableCommand, cursor, style, terminal};
use matreex::Matrix;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct Tui<F, O>
where
    F: Filter,
    O: Write,
{
    biosquare: BioSquare,
    filter: F,
    output: O,
    show_stats: bool,
    frame_timer: Timer,
    global_timer: Timer,
}

impl<F, O> Tui<F, O>
where
    F: Filter,
    O: Write,
{
    pub fn new(genesis: Matrix<Cell>, filter: F, output: O, show_stats: bool) -> Self {
        Self {
            biosquare: BioSquare::new(genesis),
            filter,
            output,
            show_stats,
            frame_timer: Timer::start(),
            global_timer: Timer::start(),
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

    pub fn run(&mut self) -> Result<&mut Self> {
        self.enter_alternate_screen()?;

        // TODO: exits loop on user input
        loop {
            self.render()?;
            self.frame_timer.reset();
            self.biosquare.evolve();
        }

        self.leave_alternate_screen()?;

        Ok(self)
    }

    pub fn render(&mut self) -> Result<&mut Self> {
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
        let fps = 1.0 / self.frame_timer.elapsed().duration().as_secs_f64();
        let runtime = self.global_timer.elapsed();

        self.render_measurement("Generation", format!("{generation}"))?
            .render_measurement("Population", format!("{population}"))?
            .render_measurement("Density", format!("{:.2} %", density * 100.0))?
            .render_measurement("FPS", format!("{fps:.2}"))?
            .render_measurement("Runtime", format!("{runtime}"))?;

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
}

impl<F, O> Tui<F, O>
where
    F: Filter,
    O: Write,
{
    fn enter_alternate_screen(&mut self) -> Result<&mut Self> {
        self.output
            .queue(cursor::Hide)?
            .queue(terminal::SetTitle("Lifegame"))?
            .queue(terminal::EnterAlternateScreen)?
            .queue(terminal::DisableLineWrap)?
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
