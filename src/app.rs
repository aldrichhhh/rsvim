use std::fs;
use std::path::PathBuf;

pub enum Mode {
    Read,
    Edit,
}

#[derive(Default, Debug)]
pub struct FileContents {
	pub filename: Option<PathBuf>,
    pub contents: Vec<String>
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
				.map(|row| {row.into()})
				.collect()
		}
	}
}

pub struct App {
    pub current_mode: Mode,
	pub file: FileContents,
}

impl App {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self {
            current_mode: Mode::Read,
			file: FileContents::new(path)
        }
    }
}
