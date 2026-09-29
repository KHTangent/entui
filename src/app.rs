use std::collections::VecDeque;
use std::future::Future;

use color_eyre::Result;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tui_input::backend::crossterm::EventHandler;

use crate::actions::Action;
use crate::components::departure_list::{DepartureList, DepartureListState};
use crate::components::list::Selectable;
use crate::components::quay_list::{QuayList, QuayListState};
use crate::components::stop_list::{StopList, StopListState};
use crate::components::suggestion_list::{SuggestionList, SuggestionListState};
use crate::components::time_editor::{self, TimeEditor, TimeEditorState, cursor_position};
use crate::entur_api_wrapper::departure_board::{DepartureBoardData, Stop, get_departures};
use crate::entur_api_wrapper::error::{ApiError, ApiResult};
use crate::entur_api_wrapper::stop_register::StopSearchResult;
use crate::events::{Event, Events};
use crate::styles;

const MAX_SUGGESTION_ROWS: u16 = 10;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AppState {
	#[default]
	EditSearch,
	DepartureList,
	BrowseStops,
	BrowseQuays,
	EditTime,
}

impl AppState {
	pub fn is_interactive(self) -> bool {
		matches!(self, AppState::EditSearch | AppState::EditTime)
	}

	pub fn keybinds(self) -> &'static [&'static str] {
		match self {
			AppState::EditSearch => &[
				"<Tab>      search",
				"<Enter>    select stop",
				"<Up/C-u>   previous suggestion",
				"<Down/C-d> next suggestion",
				"<Esc>      back",
				"<?>        toggle help",
			],
			AppState::DepartureList => &[
				"<j/Down/C-d> move down",
				"<k/Up/C-u>   move up",
				"<e>          edit search",
				"<p>          browse quays",
				"<t>          set departure time",
				"<Enter>      view stops",
				"<Esc>        clear selection",
				"<h/?>        toggle help",
				"<q>          quit",
			],
			AppState::BrowseStops => &[
				"<j/Down/C-d> move down",
				"<k/Up/C-u>   move up",
				"<Esc>        back",
				"<h/?>        toggle help",
				"<q>          quit",
			],
			AppState::BrowseQuays => &[
				"<j/Down/C-d> move down",
				"<k/Up/C-u>   move up",
				"<Enter>      filter quay",
				"<Esc>        cancel",
				"<h/?>        toggle help",
				"<q>          quit",
			],
			AppState::EditTime => &[
				"<Tab/S-Tab> next/prev field",
				"<Enter>     set time",
				"<Esc>       cancel",
				"<?>         toggle help",
			],
		}
	}
}

#[derive(Clone, Debug)]
enum FetchResult {
	Autocomplete(Vec<StopSearchResult>),
	Departures(DepartureBoardData),
	Stops(Vec<Stop>),
	Error(ApiError),
}

pub struct App {
	current_state: AppState,
	departure_list_state: DepartureListState,
	stop_list_state: StopListState,
	quay_list_state: QuayListState,
	active_errors: VecDeque<(String, String)>,
	stop_input: tui_input::Input,
	selected_stop_id: Option<String>,
	selected_quay_id: Option<String>,
	suggestion_list_state: SuggestionListState,
	time_editor_state: TimeEditorState,
	should_quit: bool,
	fetch_tx: Option<UnboundedSender<FetchResult>>,
	show_help: bool,
}

impl App {
	pub fn new() -> Self {
		Self {
			current_state: AppState::default(),
			departure_list_state: DepartureListState::new(),
			stop_list_state: StopListState::new(),
			quay_list_state: QuayListState::new(),
			active_errors: VecDeque::new(),
			selected_stop_id: None,
			selected_quay_id: None,
			stop_input: tui_input::Input::default(),
			suggestion_list_state: SuggestionListState::new(),
			time_editor_state: TimeEditorState::new(),
			should_quit: false,
			fetch_tx: None,
			show_help: true,
		}
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
							let state_before = self.current_state;
							let action = Action::from_event(&event, self.current_state);
							self.handle_action(action);
							if self.current_state == AppState::EditSearch && action != Action::SelectSearch {
								self.stop_input.handle_event(&event);
							} else if state_before == AppState::EditTime && action == Action::None {
								self.time_editor_state.handle_event(&event);
							}
						}
						Event::Error => {}
					}
				}
				Some(result) = fetch_rx.recv() => {
					match result {
						FetchResult::Departures(board) => {
							self.departure_list_state.set_departures(board.departures);
							self.departure_list_state.set_quays(&board.quays);
							self.quay_list_state.set_items(board.quays);
							self.selected_quay_id = None;
							self.stop_list_state.clear();
						}
						FetchResult::Stops(stops) => {
							let selected_index = if let Some(departure) = self.departure_list_state.selected_departure() {
								let current_quay_id = &departure.quay_id;
								stops.iter().position(|s| s.quay_id == *current_quay_id)
							} else {
								None
							};
							self.stop_list_state.set_items(stops);
							self.stop_list_state.set_selected_index(selected_index);
						}
						FetchResult::Autocomplete(results) => {
							self.suggestion_list_state.set_items(results);
						}
						FetchResult::Error(e) => {
							self.active_errors.push_back((
								format!("{:?}", e.kind),
								e.message,
							));
						},
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
		if !self.active_errors.is_empty() && action == Action::Confirm {
			self.active_errors.pop_front();
			return;
		}
		if action == Action::ToggleHelp {
			self.show_help = !self.show_help;
			return;
		}
		match action {
			Action::MoveDown => {
				self.active_selection().select_next();
				return;
			}
			Action::MoveUp => {
				self.active_selection().select_previous();
				return;
			}
			_ => {}
		}
		match self.current_state {
			AppState::DepartureList => match action {
				Action::Quit => {
					self.should_quit = true;
				}
				Action::Cancel => {
					self.departure_list_state.deselect();
					self.departure_list_state.clear_quay_filter();
					self.selected_quay_id = None;
					self.stop_list_state.clear();
				}
				Action::SelectSearch => {
					self.current_state = AppState::EditSearch;
				}
				Action::SelectQuay => {
					self.current_state = AppState::BrowseQuays;
				}
				Action::SelectTime => {
					self.time_editor_state.reset();
					self.current_state = AppState::EditTime;
				}
				Action::Confirm if self.departure_list_state.selected_departure().is_some() => {
					self.populate_stops();
					self.current_state = AppState::BrowseStops;
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
				_ => {}
			},
			AppState::BrowseQuays => match action {
				Action::Cancel => {
					self.departure_list_state.clear_quay_filter();
					self.selected_quay_id = None;
					self.current_state = AppState::DepartureList;
				}
				Action::Quit => {
					self.should_quit = true;
				}
				Action::Confirm => {
					if let Some(quay) = self.quay_list_state.selected().cloned() {
						self.departure_list_state.set_quay_filter(Some(&quay.id));
						self.selected_quay_id = Some(quay.id);
						self.current_state = AppState::DepartureList;
					}
				}
				_ => {}
			},
			AppState::EditSearch => match action {
				Action::Cancel => {
					if !self.departure_list_state.is_empty() {
						self.current_state = AppState::DepartureList;
					}
				}
				Action::ManualSearch => {
					self.populate_autocomplete();
				}
				Action::Confirm => {
					if let Some(suggestion) = self.suggestion_list_state.selected().cloned() {
						self.stop_input = tui_input::Input::new(suggestion.label);
						self.selected_stop_id = Some(suggestion.id);
						self.populate_departures();
						self.current_state = AppState::DepartureList;
					}
				}
				_ => {}
			},
			AppState::EditTime => match action {
				Action::NextField => {
					self.time_editor_state.next_field();
				}
				Action::PreviousField => {
					self.time_editor_state.previous_field();
				}
				Action::Cancel => {
					self.current_state = AppState::DepartureList;
				}
				Action::Confirm => match self.time_editor_state.parse() {
					Ok(timestamp) => {
						tracing::info!(%timestamp, "selected departure time");
						self.current_state = AppState::DepartureList;
					}
					Err(message) => {
						self.active_errors
							.push_back(("Invalid time".to_string(), message));
					}
				},
				_ => {}
			},
		}
	}

	fn active_selection(&mut self) -> &mut dyn Selectable {
		match self.current_state {
			AppState::DepartureList => &mut self.departure_list_state,
			AppState::BrowseStops => &mut self.stop_list_state,
			AppState::BrowseQuays => &mut self.quay_list_state,
			AppState::EditSearch => &mut self.suggestion_list_state,
			AppState::EditTime => &mut self.time_editor_state,
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
					Style::new().fg(if self.current_state == AppState::EditSearch {
						styles::ACTIVE_COLOR
					} else {
						styles::INACTIVE_COLOR
					}),
				)
				.title_bottom("Stop name"),
		);
		frame.render_widget(search_text, search_bar_rect);
		if self.current_state == AppState::EditSearch {
			let x = self.stop_input.visual_cursor() as u16;
			frame.set_cursor_position((search_bar_rect.x + x + 2, search_bar_rect.y + 2_u16));
		}

		frame.render_stateful_widget(
			DepartureList::new().with_focused(self.current_state == AppState::DepartureList),
			departures_rect,
			&mut self.departure_list_state,
		);

		if self.current_state == AppState::BrowseQuays {
			frame.render_stateful_widget(
				QuayList::new().with_focused(true),
				details_rect,
				&mut self.quay_list_state,
			);
		} else if self.departure_list_state.selected_departure().is_some() {
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

		if self.current_state == AppState::EditTime {
			let editor_area = frame.area().centered(
				Constraint::Length(time_editor::EDITOR_WIDTH),
				Constraint::Length(time_editor::EDITOR_HEIGHT),
			);
			frame.render_widget(Clear, editor_area);
			frame.render_stateful_widget(TimeEditor, editor_area, &mut self.time_editor_state);
			frame.set_cursor_position(cursor_position(editor_area, &self.time_editor_state));
		}

		if let Some((error_title, error_description)) = self.active_errors.front() {
			let error_area = frame.area().centered(
				Constraint::Length(error_description.len() as u16 + 6),
				Constraint::Length(7),
			);
			let error_paragraph = Paragraph::new(error_description.as_str()).block(
				Block::default()
					.borders(Borders::ALL)
					.padding(Padding::uniform(2))
					.border_style(Style::new().fg(styles::ACTIVE_COLOR))
					.title_top(error_title.as_str())
					.title_bottom(Line::from("<Enter>").centered()),
			);
			frame.render_widget(Clear, error_area);
			frame.render_widget(error_paragraph, error_area);
		}

		if self.show_help {
			self.render_floating_help(frame);
		}
	}

	fn render_floating_help(&mut self, frame: &mut Frame) {
		let keybinds = self.current_state.keybinds();
		let content_width = keybinds
			.iter()
			.map(|k| k.chars().count())
			.max()
			.unwrap_or(0) as u16;
		let area = frame.area();
		let help_rect = Rect {
			x: area.right().saturating_sub(content_width + 4),
			y: area.bottom().saturating_sub(keybinds.len() as u16 + 2),
			width: content_width + 4,
			height: keybinds.len() as u16 + 2,
		};
		let lines = keybinds.iter().map(|k| Line::from(*k)).collect::<Vec<_>>();
		let help = Paragraph::new(lines).block(
			Block::default()
				.borders(Borders::ALL)
				.padding(Padding::horizontal(1))
				.border_style(Style::new().fg(styles::INACTIVE_COLOR))
				.title_bottom("Help"),
		);
		frame.render_widget(Clear, help_rect);
		frame.render_widget(help, help_rect);
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

	fn spawn_fetch<T>(
		&self,
		fetch: impl Future<Output = ApiResult<T>> + Send + 'static,
		on_ok: fn(T) -> FetchResult,
	) where
		T: Send + 'static,
	{
		if let Some(tx) = &self.fetch_tx {
			let tx = tx.clone();
			tokio::spawn(async move {
				let _ = match fetch.await {
					Ok(value) => tx.send(on_ok(value)),
					Err(e) => tx.send(FetchResult::Error(e)),
				};
			});
		}
	}

	fn populate_autocomplete(&mut self) {
		let query = self.stop_input.value().to_string();
		self.spawn_fetch(
			async move { StopSearchResult::search(&query).await },
			FetchResult::Autocomplete,
		);
	}

	fn populate_departures(&mut self) {
		if let Some(from) = self.selected_stop_id.clone() {
			self.spawn_fetch(
				async move { get_departures(&from).await },
				FetchResult::Departures,
			);
		}
	}

	fn populate_stops(&mut self) {
		if let Some(departure) = self.departure_list_state.selected_departure().cloned() {
			self.spawn_fetch(
				async move { departure.get_stops().await },
				FetchResult::Stops,
			);
		}
	}
}
