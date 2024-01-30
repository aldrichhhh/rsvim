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
    })?;

	match app.events.receiver.recv()? {
		UserEvent::Key(event) => {
			match event {
				KeyEvent {
					code: KeyCode::Char('q'),
					modifiers: KeyModifiers::CONTROL,
					..
				} => break,
				KeyEvent {
					code: KeyCode::Char('a'),
					modifiers: KeyModifiers::NONE,
					..
				} => app.file.contents.push(Row::new(String::from("a"))),
				_ => {}
			}
		}
		_ => {}
	}
  }
    Ok(())
}