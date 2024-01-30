use std::io;
use std::{fs, io::Stdout};
use std::path::PathBuf;

use ratatui::backend::{self, CrosstermBackend, WindowSize};

use crate::event::EventHandler;

pub enum Mode {
    Read,
    Edit,
}

#[derive(Debug)]
pub struct Row {
	pub row_content: String,
}

impl Row {
	pub fn new(row_content: String) -> Self {
		Self { row_content }
	}
}

#[derive(Default, Debug)]
pub struct FileContents {
	pub filename: Option<PathBuf>,
    pub contents: Vec<Row>
}

impl FileContents {
    pub fn new(path: Option<PathBuf>) -> Self {
		match path {
			None => Self {
				filename: None,
				contents: Vec::new()
			},
			Some(filename) => Self::from_file(filename)
		}
    }

	fn from_file(file: PathBuf) -> Self {
		let contents = fs::read_to_string(&file).expect("unable to read file");
		Self {
			filename: Some(file),
			contents: contents
				.lines()
				.map(|row| {
					Row::new(row.into())
				})
				.collect()
		}
	}
}

pub struct App {
    pub current_mode: Mode,
	pub file: FileContents,
	pub events: EventHandler,
}

impl App {
    pub fn new(path: Option<PathBuf>, events: EventHandler) -> Self {
        Self {
            current_mode: Mode::Read,
			file: FileContents::new(path),
			events,
        }
    }
}
