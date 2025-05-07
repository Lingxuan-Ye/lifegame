use anyhow::Error;
use crossterm::style::Stylize;
use crossterm::{QueueableCommand, style};
use std::io::{self, Write, stderr};
use std::process::exit;

pub trait QuitOnError<T>: internal::Sealed {
    fn quit_on_error(self) -> T;
}

impl<T, E> QuitOnError<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn quit_on_error(self) -> T {
        match self {
            Err(error) => {
                eprint(error);
                exit(1);
            }
            Ok(value) => value,
        }
    }
}

fn eprint<E>(error: E)
where
    E: Into<Error>,
{
    if let Err(io_error) = try_eprint(error) {
        panic!("failed printing to stderr: {}", io_error);
    }
}

fn try_eprint<E>(error: E) -> io::Result<()>
where
    E: Into<Error>,
{
    let mut stderr = stderr().lock();

    let error_label = "error:".red().bold();
    let caused_by_label = "caused by:".red().bold();

    for (index, cause) in error.into().chain().enumerate() {
        let message = if index == 0 {
            format!("{error_label} {cause}")
        } else {
            format!("{caused_by_label} {cause}")
        };
        stderr
            .queue(style::Print(message))?
            .queue(style::Print("\n"))?;
    }

    stderr.flush()
}

mod internal {
    use super::Error;

    pub trait Sealed {}
    impl<T, E> Sealed for Result<T, E> where E: Into<Error> {}
}
