use color_eyre::Result;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tui_input::backend::crossterm::EventHandler;

use crate::actions::Action;
use crate::components::departure_list::{DepartureList, DepartureListState};
use crate::components::stop_list::{StopList, StopListState};
use crate::components::suggestion_list::{SuggestionList, SuggestionListState};
use crate::entur_api_wrapper::departure_board::{Departure, Stop, get_departures};
use crate::events::{Event, Events};
use crate::styles;

const MAX_SUGGESTION_ROWS: u16 = 10;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AppState {
	#[default]
	EditSearch,
	DepartureList,
	BrowseStops,
}

#[derive(Clone, Debug)]
enum FetchResult {
	Departures(Vec<Departure>),
	Stops(Vec<Stop>, String),
}

pub struct App {
	current_state: AppState,
	departure_list_state: DepartureListState,
	stop_list_state: StopListState,
	stop_input: tui_input::Input,
	suggestion_list_state: SuggestionListState,
	should_quit: bool,
	fetch_tx: Option<UnboundedSender<FetchResult>>,
}

impl App {
	pub fn new() -> Self {
		let mut app = Self {
			current_state: AppState::default(),
			departure_list_state: DepartureListState::new(),
			stop_list_state: StopListState::new(),
			stop_input: tui_input::Input::default(),
			suggestion_list_state: SuggestionListState::new(),
			should_quit: false,
			fetch_tx: None,
		};
		app.suggestion_list_state.set_suggestions(vec![
			"Siemens".to_string(),
			"Trondheim S".to_string(),
			"Nidarosdomen".to_string(),
			"Lerkendal".to_string(),
			"Strindheim".to_string(),
			"Sæterbakken".to_string(),
			"Studentersamfundet".to_string(),
			"Prinsens gate".to_string(),
			"Nidarvoll skole".to_string(),
			"Astronomvegen".to_string(),
			"Buran 2".to_string(),
			"Rønningsbakken".to_string(),
		]);
		app
	}

	pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
		let (fetch_tx, mut fetch_rx) = unbounded_channel();
		self.fetch_tx = Some(fetch_tx);
		let mut events = Events::new();
		loop {
			tokio::select! {
				Some(event) = events.next() => {
					match event {
						Event::Render => {
							terminal.draw(|frame| self.render(frame))?;
						}
						Event::Crossterm(event) => {
							let action = Action::from_event(&event, self.current_state);
							self.handle_action(action);
							if self.current_state == AppState::EditSearch {
								self.stop_input.handle_event(&event);
							}
						}
						Event::Error => {}
					}
				}
				Some(result) = fetch_rx.recv() => {
					match result {
						FetchResult::Departures(departures) => {
							self.departure_list_state.set_departures(departures);
							self.stop_list_state.clear();
						}
						FetchResult::Stops(stops, search_name) => {
							let selected_index = stops.iter().position(|s| s.name == search_name).or(None);
							self.stop_list_state.set_stops(stops);
							self.stop_list_state.set_selected_index(selected_index);
						}
					}
				}
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
						self.populate_stops();
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
				Action::MoveDown => {
					self.suggestion_list_state.select_next();
				}
				Action::MoveUp => {
					self.suggestion_list_state.select_previous();
				}
				Action::Confirm => {
					if let Some(suggestion) =
						self.suggestion_list_state.selected_suggestion().cloned()
					{
						self.stop_input = tui_input::Input::new(suggestion);
						self.populate_departures();
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

		if self.current_state == AppState::EditSearch && !self.stop_input.value().is_empty() {
			self.render_suggestions(frame, search_bar_rect);
		}
	}

	fn render_suggestions(&mut self, frame: &mut Frame, anchor: Rect) {
		let row_count = (self.suggestion_list_state.len() as u16).min(MAX_SUGGESTION_ROWS);
		let popup_height = row_count.saturating_add(2);
		let popup_rect = Rect {
			x: anchor.x,
			y: anchor.y.saturating_sub(popup_height),
			width: anchor.width,
			height: popup_height,
		};
		frame.render_widget(Clear, popup_rect);
		frame.render_stateful_widget(
			SuggestionList::new().with_focused(true),
			popup_rect,
			&mut self.suggestion_list_state,
		);
	}

	fn populate_departures(&mut self) {
		let from = self.stop_input.value().to_string();
		if let Some(tx) = &self.fetch_tx {
			let tx = tx.clone();
			tokio::spawn(async move {
				let departures = get_departures(&from).await;
				let _ = tx.send(FetchResult::Departures(departures));
			});
		}
	}

	fn populate_stops(&mut self) {
		if let Some(departure) = self.departure_list_state.selected_departure().cloned() {
			let search_name = self.stop_input.value().to_string();
			if let Some(tx) = &self.fetch_tx {
				let tx = tx.clone();
				tokio::spawn(async move {
					let stops = departure.get_stops().await;
					let _ = tx.send(FetchResult::Stops(stops, search_name));
				});
			}
		}
	}
}
