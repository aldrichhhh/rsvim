use ratatui::{backend::Backend, layout::{Constraint, Direction, Layout}, widgets::Paragraph, Terminal};
use anyhow::Result;

pub mod app;
use app::App;

pub fn start_app<B: Backend>(
  terminal: &mut Terminal<B>,
  app: &mut App
) -> Result<()> {
  loop {
    terminal.draw(|frame| {
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(98), Constraint::Percentage(2)])
        .split(frame.size());
      let content = Paragraph::new(format!("{:#?}", app.file.contents));
      frame.render_widget(content, chunks[0]);
    });
  }
    Ok(())
}