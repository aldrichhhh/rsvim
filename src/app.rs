use std::path::PathBuf;
use std::{cmp, io};
use std::{fs, io::Stdout};

use crossterm::event::KeyCode;
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

    pub fn insert_char(&mut self, idx: usize, ch: char) {
        self.row_content.insert(idx, ch);
    }

    pub fn delete_char(&mut self, idx: usize) {
        self.row_content.remove(idx);
    }

    pub fn length(&self) -> usize {
        self.row_content.len()
    }
}

impl ToString for Row {
    fn to_string(&self) -> String {
        self.row_content.clone()
    }
}

#[derive(Default, Debug)]
pub struct FileContents {
    pub filename: Option<PathBuf>,
    pub contents: Vec<Row>,
}

impl FileContents {
    pub fn new(path: Option<PathBuf>) -> Self {
        match path {
            None => Self {
                filename: None,
                contents: Vec::new(),
            },
            Some(filename) => Self::from_file(filename),
        }
    }

    fn from_file(file: PathBuf) -> Self {
        let contents = fs::read_to_string(&file).expect("unable to read file");
        Self {
            filename: Some(file),
            contents: contents.lines().map(|row| Row::new(row.into())).collect(),
        }
    }

    pub fn get_row(&mut self, idx: usize) -> &mut Row {
        &mut self.contents[idx]
    }
}

pub struct Cursor {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub max_x: usize,
    pub max_y: usize,
}

impl Cursor {
    fn new(win_size: &WindowSize) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            max_x: win_size.columns_rows.width as usize,
            max_y: (win_size.columns_rows.height - 2) as usize,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode, contents: &mut FileContents) {
        match direction {
            KeyCode::Left => {
                if self.cursor_x == 0 && self.cursor_y != 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = contents.get_row(self.cursor_y).length()
                } else {
                    self.cursor_x = self.cursor_x.saturating_sub(1);
                }
            }
            KeyCode::Right => {
                if self.cursor_x < contents.get_row(self.cursor_y).length() {
                    self.cursor_x += 1;
                }
                // Wrap to the next line
                else if self.cursor_y < self.max_y {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
            }
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Down => {
                if self.cursor_y < self.max_y {
                    self.cursor_y += 1;
                }
            }
            _ => {}
        }
        let max_len = if self.cursor_y < self.max_y {
            contents.get_row(self.cursor_y).length()
        } else {
            0
        };
        self.cursor_x = cmp::min(self.cursor_x, max_len);
    }
}

pub struct App {
    pub current_mode: Mode,
    pub file: FileContents,
    pub events: EventHandler,
    pub cursor: Cursor,
}

impl App {
    pub fn new(path: Option<PathBuf>, events: EventHandler, window_size: &WindowSize) -> Self {
        Self {
            current_mode: Mode::Read,
            file: FileContents::new(path),
            events,
            cursor: Cursor::new(window_size),
        }
    }
}
