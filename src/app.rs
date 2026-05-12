use color_eyre::Result;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tui_input::backend::crossterm::EventHandler;

use crate::actions::Action;
use crate::components::departure_list::{DepartureList, DepartureListState};
use crate::components::stop_list::{StopList, StopListState};
use crate::entur_api_wrapper::departure_board::get_departures;
use crate::events::{Event, Events};
use crate::styles;

#[derive(PartialEq, Eq, Default)]
enum AppState {
	#[default]
	EditSearch,
	DepartureList,
	BrowseStops,
}

pub struct App {
	current_state: AppState,
	departure_list_state: DepartureListState,
	stop_list_state: StopListState,
	stop_input: tui_input::Input,
	should_quit: bool,
}

impl App {
	pub fn new() -> Self {
		Self {
			current_state: AppState::default(),
			departure_list_state: DepartureListState::new(),
			stop_list_state: StopListState::new(),
			stop_input: tui_input::Input::default(),
			should_quit: false,
		}
	}

	#[tokio::main]
	pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
		let mut events = Events::new();
		loop {
			match events.next().await {
				Some(Event::Render) => {
					terminal.draw(|frame| self.render(frame))?;
				}
				Some(Event::Crossterm(event)) => {
					let action = Action::from_event(&event);
					self.handle_action(action);
					if self.current_state == AppState::EditSearch {
						self.stop_input.handle_event(&event);
					}
				}
				Some(Event::Error) => {}
				None => todo!(),
			}
			if self.should_quit {
				return Ok(());
			}
		}
	}

	fn handle_action(&mut self, action: Action) {
		if action == Action::HardQuit {
			self.should_quit = true;
			return;
		}
		match self.current_state {
			AppState::DepartureList => match action {
				Action::Quit => {
					self.should_quit = true;
				}
				Action::Cancel => {
					self.departure_list_state.deselect();
					self.stop_list_state.clear();
				}
				Action::SelectSearch => {
					self.current_state = AppState::EditSearch;
				}
				Action::MoveDown => {
					self.departure_list_state.select_next();
				}
				Action::MoveUp => {
					self.departure_list_state.select_previous();
				}
				Action::Confirm => {
					if self.departure_list_state.selected_departure().is_some() {
						self.initialize_browse_stops();
						self.current_state = AppState::BrowseStops;
					}
				}
				_ => {}
			},
			AppState::BrowseStops => match action {
				Action::Cancel => {
					self.current_state = AppState::DepartureList;
				}
				Action::Quit => {
					self.should_quit = true;
				}
				Action::MoveDown => {
					self.stop_list_state.select_next();
				}
				Action::MoveUp => {
					self.stop_list_state.select_previous();
				}
				_ => {}
			},
			AppState::EditSearch => match action {
				Action::Cancel => {
					if !self.departure_list_state.is_empty() {
						self.current_state = AppState::DepartureList;
					}
				}
				Action::Confirm => {
					self.initialize_departures();
					if !self.departure_list_state.is_empty() {
						self.current_state = AppState::DepartureList;
					}
				}
				_ => {}
			},
		}
	}

	fn render(&mut self, frame: &mut Frame) {
		let [main_layout_rect, search_bar_rect] = frame.area().layout(&Layout::vertical([
			Constraint::Fill(1),
			Constraint::Length(5),
		]));
		let [departures_rect, details_rect] = main_layout_rect.layout(&Layout::horizontal([
			Constraint::Fill(1),
			Constraint::Fill(1),
		]));

		let search_text = Paragraph::new(self.stop_input.value()).block(
			Block::default()
				.borders(Borders::ALL)
				.padding(Padding::uniform(1))
				.border_style(
					Style::new().fg((self.current_state == AppState::EditSearch)
						.then_some(styles::ACTIVE_COLOR)
						.unwrap_or(styles::INACTIVE_COLOR)),
				)
				.title_bottom("Stop name"),
		);
		frame.render_widget(search_text, search_bar_rect);
		if self.current_state == AppState::EditSearch {
			let x = self.stop_input.visual_cursor() as u16;
			frame.set_cursor_position((search_bar_rect.x + x + 2, search_bar_rect.y + 2 as u16));
		}

		frame.render_stateful_widget(
			DepartureList::new().with_focused(self.current_state == AppState::DepartureList),
			departures_rect,
			&mut self.departure_list_state,
		);

		if self.departure_list_state.selected_departure().is_some() {
			frame.render_stateful_widget(
				StopList::new().with_focused(self.current_state == AppState::BrowseStops),
				details_rect,
				&mut self.stop_list_state,
			);
		} else {
			let details_dummy = Block::new().borders(Borders::ALL);
			frame.render_widget(details_dummy, details_rect);
		}
	}

	fn initialize_departures(&mut self) {
		self.departure_list_state
			.set_departures(get_departures(self.stop_input.value()));
		self.stop_list_state.clear();
	}

	fn initialize_browse_stops(&mut self) {
		if let Some(departure) = self.departure_list_state.selected_departure() {
			let stops = departure.get_stops();
			let search_name = self.stop_input.value();

			let selected_index = stops.iter().position(|s| s.name == search_name).or(None);

			self.stop_list_state.set_stops(stops);
			self.stop_list_state.set_selected_index(selected_index);
		}
	}
}
