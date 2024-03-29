use anyhow::Result;
use clap::Parser;
use crossterm::cursor;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use rsvim::app::App;
use rsvim::event::EventHandler;
use rsvim::start_app;
use std::io::stdout;
use std::{io, path::PathBuf};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)
            .expect("failed to exit alternate screen");
    }
}

#[derive(Parser, Debug)]
#[command(version, author, about = "Custom vim editor built in rust", long_about)]
struct Args {
    /// Filename to edit
    filename: Option<PathBuf>,
}

fn main() -> Result<()> {
    let _clean_up = CleanUp;
    let args = Args::parse();

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let events = EventHandler::new(250);
    let mut backend = CrosstermBackend::new(std::io::stdout());
    let mut app = App::new(args.filename, events, &backend.window_size().unwrap());
    let mut terminal = Terminal::new(backend)?;
    start_app(&mut terminal, &mut app)?;

    Ok(())
}
