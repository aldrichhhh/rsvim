use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug)]
pub enum UserEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16)
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
  pub sender: mpsc::Sender<UserEvent>,
  pub receiver: mpsc::Receiver<UserEvent>,
  pub handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        // let tick_rate = Duration::from_millis(tick_rate);
		let (sender, receiver) = mpsc::channel();
		let handler = {
			let sender = sender.clone();
			thread::spawn(move || loop {
				if let Event::Key(event) = event::read().expect("failed to read events") {
					sender.send(UserEvent::Key(event)).expect("");
				}
			})
		};

		Self {
			sender,
			receiver,
			handler,
		}
    }
}