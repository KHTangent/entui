use ratatui::crossterm::event::{Event as CrosstermEvent, KeyCode, KeyModifiers};

use crate::app::AppState;

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
	pub fn from_event(e: &CrosstermEvent, current_state: AppState) -> Action {
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

					KeyCode::Char('k') if current_state != AppState::EditSearch => Action::MoveUp,
					KeyCode::Up => Action::MoveUp,
					KeyCode::Char('p') if key.modifiers == KeyModifiers::CONTROL => Action::MoveUp,

					KeyCode::Char('j') if current_state != AppState::EditSearch => Action::MoveDown,
					KeyCode::Down => Action::MoveDown,
					KeyCode::Char('n') if key.modifiers == KeyModifiers::CONTROL => {
						Action::MoveDown
					}

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
