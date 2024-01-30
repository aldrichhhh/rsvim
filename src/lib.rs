use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use event::UserEvent;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Paragraph, Wrap},
    Terminal,
};

pub mod app;
pub mod event;
use app::{App, Row};

pub fn start_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(98), Constraint::Percentage(2)])
                .split(frame.size());
            let content = Paragraph::new(
                app.file
                    .contents
                    .iter()
                    .map(|row| row.to_string() + "\r\n")
                    .collect::<String>(),
            )
            .wrap(Wrap { trim: false });

            frame.render_widget(Paragraph::new("Mode: Write"), chunks[1]);
            frame.render_widget(content, chunks[0]);
            frame.set_cursor(app.cursor.cursor_x as u16, app.cursor.cursor_y as u16);
        })?;

        match app.events.receiver.recv()? {
            UserEvent::Key(event) => match event {
                KeyEvent {
                    code: code @ (KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down),
                    ..
                } => app.cursor.move_cursor(code, &mut app.file),
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    if app.cursor.cursor_x > 0 {
                        app.file
                            .get_row(app.cursor.cursor_y)
                            .delete_char(app.cursor.cursor_x - 1);
                        app.cursor.cursor_x -= 1;
                    }
                }
                KeyEvent {
                    code: KeyCode::Delete,
                    ..
                } => {
                    if app.cursor.cursor_x < app.file.get_row(app.cursor.cursor_y).length() {
                        app.file
                            .get_row(app.cursor.cursor_y)
                            .delete_char(app.cursor.cursor_x)
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => break,
                KeyEvent {
                    code: KeyCode::Char(ch),
                    ..
                } => {
                    if app.cursor.cursor_x <= app.file.get_row(app.cursor.cursor_y).length() {
                        app.file
                            .get_row(app.cursor.cursor_y)
                            .insert_char(app.cursor.cursor_x, ch);
                        app.cursor.cursor_x += 1;
                    }
                }
                _ => {}
            },
            UserEvent::Mouse(_mouseevent) => {}
            _ => {}
        }
    }
    Ok(())
}
