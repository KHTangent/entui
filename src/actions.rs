use ratatui::crossterm::event::{Event as CrosstermEvent, KeyCode, KeyModifiers};

use crate::app::AppState;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
	None,
	MoveDown,
	MoveUp,
	SelectSearch,
	ManualSearch,
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
				match (current_state, key.code, key.modifiers) {
					(_, KeyCode::Char('c'), KeyModifiers::CONTROL) => Action::HardQuit,
					(_, KeyCode::Char('q'), KeyModifiers::NONE) => Action::Quit,

					(_, KeyCode::Char('k'), KeyModifiers::NONE)
						if current_state != AppState::EditSearch =>
					{
						Action::MoveUp
					}
					(_, KeyCode::Up, KeyModifiers::NONE) => Action::MoveUp,
					(_, KeyCode::Char('p'), KeyModifiers::CONTROL) => Action::MoveUp,

					(_, KeyCode::Char('j'), KeyModifiers::NONE)
						if current_state != AppState::EditSearch =>
					{
						Action::MoveDown
					}
					(_, KeyCode::Down, KeyModifiers::NONE) => Action::MoveDown,
					(_, KeyCode::Char('n'), KeyModifiers::CONTROL) => Action::MoveDown,

					(AppState::DepartureList, KeyCode::Char('e'), KeyModifiers::NONE) => {
						Action::SelectSearch
					}
					(_, KeyCode::Enter, KeyModifiers::NONE) => Action::Confirm,
					(_, KeyCode::Tab, KeyModifiers::NONE) => Action::ManualSearch,
					(_, KeyCode::Esc, KeyModifiers::NONE) => Action::Cancel,

					(_, _, _) => Action::None,
				}
			}
			_ => Action::None,
		}
	}
}
