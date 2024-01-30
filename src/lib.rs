use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use event::UserEvent;
use ratatui::{backend::Backend, layout::{Constraint, Direction, Layout}, widgets::Paragraph, Terminal};
use anyhow::Result;

pub mod app;
pub mod event;
use app::{App, Row};

pub fn start_app<B: Backend>(
  terminal: &mut Terminal<B>,
  app: &mut App,
) -> Result<()> {
  loop {
    terminal.draw(|frame| {
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(98), Constraint::Percentage(2)])
        .split(frame.size());
      let content = Paragraph::new(format!("{:#?}", app.file.contents));
      frame.render_widget(content, chunks[0]);
	  frame.set_cursor(app.cursor.cursor_x as u16, app.cursor.cursor_y as u16);
    })?;

	match app.events.receiver.recv()? {
		UserEvent::Key(event) => {
			match event {
				KeyEvent {
					code: code @ (KeyCode::Left | KeyCode::Right
					| KeyCode::Up | KeyCode::Down),
					..
				} => app.cursor.move_cursor(code),
				KeyEvent {
					code: KeyCode::Char('q'),
					modifiers: KeyModifiers::CONTROL,
					..
				} => break,
				_ => {}
			}
		}
		UserEvent::Mouse(_mouseevent) => {}
		_ => {}
	}
  }
    Ok(())
}