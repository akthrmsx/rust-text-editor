use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType},
    Command,
};
use std::{
    fmt::Display,
    io::{self, Error, Write},
};

#[derive(Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub column: usize,
    pub row: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.column as u16, position.row as u16))?;
        Ok(())
    }

    pub fn print<T: Display>(value: T) -> Result<(), Error> {
        Self::queue_command(Print(value))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        io::stdout().flush()?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = terminal::size()?;
        let height = height as usize;
        let width = width as usize;
        Ok(Size { height, width })
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(io::stdout(), command)?;
        Ok(())
    }
}
