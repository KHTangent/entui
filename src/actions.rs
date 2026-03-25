use ratatui::crossterm::event::{Event as CrosstermEvent, KeyCode, KeyModifiers};

#[derive(PartialEq, Eq)]
pub enum Action {
	None,
	MoveDown,
	MoveUp,
	SelectSearch,
	Cancel,
	Confirm,
	Quit,
	HardQuit,
}

impl Action {
	pub fn from_event(e: &CrosstermEvent) -> Action {
		match e {
			CrosstermEvent::Key(key) => {
				if !key.is_press() {
					return Action::None;
				}
				match key.code {
					KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
						Action::HardQuit
					}
					KeyCode::Char('q') => Action::Quit,
					KeyCode::Char('k') | KeyCode::Up => Action::MoveUp,
					KeyCode::Char('j') | KeyCode::Down => Action::MoveDown,
					KeyCode::Char('e') => Action::SelectSearch,
					KeyCode::Enter => Action::Confirm,
					KeyCode::Esc => Action::Cancel,
					_ => Action::None,
				}
			}
			_ => Action::None,
		}
	}
}
