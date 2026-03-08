use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tui_input::backend::crossterm::EventHandler;

use crate::components::departure_list::DepartureList;
use crate::components::stop_list::StopList;
use crate::entur_api_wrapper::departure_board::{Departure, get_departures};
use crate::styles;

#[derive(PartialEq, Eq, Default)]
enum AppState {
	#[default]
	DepartureList,
	EditSearch,
	BrowseStops,
}

pub struct App {
	current_state: AppState,
	active_departures: Vec<Departure>,
	selected_departure_index: Option<usize>,
	stop_input: tui_input::Input,
	selected_stop_index: Option<usize>,
	stop_scroll_offset: usize,
	list_containers_height: usize,
}

impl App {
	pub fn new() -> Self {
		Self {
			current_state: AppState::default(),
			active_departures: vec![],
			stop_input: tui_input::Input::default(),
			selected_departure_index: None,
			selected_stop_index: None,
			stop_scroll_offset: 0,
			list_containers_height: 1,
		}
	}

	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
		loop {
			self.active_departures = get_departures("Siemens");

			terminal.draw(|frame| self.render(frame))?;

			let event = event::read()?;
			if let Event::Key(key) = event {
				if key.is_press()
					&& key.modifiers == KeyModifiers::CONTROL
					&& key.code == KeyCode::Char('c')
				{
					return Ok(());
				}
				match self.current_state {
					AppState::DepartureList => match key.code {
						KeyCode::Char('q') if key.kind == KeyEventKind::Press => {
							return Ok(());
						}
						KeyCode::Char('e') if key.kind == KeyEventKind::Press => {
							self.current_state = AppState::EditSearch;
						}
						KeyCode::Char('j') | KeyCode::Down if key.kind == KeyEventKind::Press => {
							self.select_next_departure();
						}
						KeyCode::Char('k') | KeyCode::Up if key.kind == KeyEventKind::Press => {
							self.select_previous_departure();
						}
						KeyCode::Esc if key.kind == KeyEventKind::Press => {
							self.deselect_departure();
						}
						KeyCode::Enter if key.kind == KeyEventKind::Press => {
							if let Some(index) = self.selected_departure_index {
								self.initialize_browse_stops(index);
								self.current_state = AppState::BrowseStops;
							}
						}
						_ => {}
					},
					AppState::BrowseStops => match key.code {
						KeyCode::Esc if key.kind == KeyEventKind::Press => {
							self.current_state = AppState::DepartureList;
						}
						KeyCode::Char('q') if key.kind == KeyEventKind::Press => {
							return Ok(());
						}
						KeyCode::Char('j') | KeyCode::Down if key.kind == KeyEventKind::Press => {
							self.select_next_stop();
						}
						KeyCode::Char('k') | KeyCode::Up if key.kind == KeyEventKind::Press => {
							self.select_previous_stop();
						}
						_ => {}
					},
					AppState::EditSearch => match key.code {
						KeyCode::Esc if key.kind == KeyEventKind::Press => {
							self.current_state = AppState::DepartureList;
						}
						_ => {
							self.stop_input.handle_event(&event);
						}
					},
				}
			}
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

		frame.render_widget(
			DepartureList::from(&self.active_departures)
				.with_focused(self.current_state == AppState::DepartureList)
				.with_selected_index(self.selected_departure_index),
			departures_rect,
		);
		self.list_containers_height = (departures_rect.height - 2).max(1) as usize;

		if let Some(selected_departure) = self.selected_departure_index {
			let stops = self.active_departures[selected_departure].get_stops();
			frame.render_widget(
				StopList::from(&stops)
					.with_focused(self.current_state == AppState::BrowseStops)
					.with_selected_index(self.selected_stop_index)
					.with_scroll_offset(self.stop_scroll_offset),
				details_rect,
			);
		} else {
			let details_dummy = Block::new().borders(Borders::ALL);
			frame.render_widget(details_dummy, details_rect);
		}
	}

	fn select_next_departure(&mut self) {
		if let Some(index) = self.selected_departure_index {
			if index + 1 < self.active_departures.len() {
				self.selected_departure_index = Some(index + 1);
			}
		} else if !self.active_departures.is_empty() {
			self.selected_departure_index = Some(0);
		}
	}

	fn select_previous_departure(&mut self) {
		if let Some(index) = self.selected_departure_index {
			if index > 0 {
				self.selected_departure_index = Some(index - 1);
			}
		} else if !self.active_departures.is_empty() {
			self.selected_departure_index = Some(self.active_departures.len() - 1);
		}
	}

	fn deselect_departure(&mut self) {
		self.selected_departure_index = None;
	}

	fn initialize_browse_stops(&mut self, departure_index: usize) {
		let stops = self.active_departures[departure_index].get_stops();
		let search_name = self.stop_input.value();

		let selected_index = stops.iter().position(|s| s.name == search_name).or(None);

		self.selected_stop_index = selected_index;
		self.stop_scroll_offset = 0;
	}

	fn select_next_stop(&mut self) {
		if let Some(departure_index) = self.selected_departure_index {
			let stops = self.active_departures[departure_index].get_stops();

			if let Some(index) = self.selected_stop_index {
				if index + 1 < stops.len() {
					self.selected_stop_index = Some(index + 1);
					self.adjust_stop_scroll(stops.len());
				}
			} else if !stops.is_empty() {
				self.selected_stop_index = Some(0);
			}
		}
	}

	fn select_previous_stop(&mut self) {
		if let Some(departure_index) = self.selected_departure_index {
			let stops = self.active_departures[departure_index].get_stops();

			if let Some(index) = self.selected_stop_index {
				if index > 0 {
					self.selected_stop_index = Some(index - 1);
					self.adjust_stop_scroll(stops.len());
				}
			} else if !stops.is_empty() {
				self.selected_stop_index = Some(stops.len() - 1);
			}
		}
	}

	fn adjust_stop_scroll(&mut self, total_stops: usize) {
		if let Some(selected) = self.selected_stop_index {
			let visible_height = self.list_containers_height;

			if selected < self.stop_scroll_offset {
				// Selected is above visible area, scroll up
				self.stop_scroll_offset = selected;
			} else if selected >= self.stop_scroll_offset + visible_height {
				// Selected is below visible area, scroll down
				self.stop_scroll_offset = selected.saturating_sub(visible_height - 1);
			}

			// Ensure scroll offset doesn't go beyond bounds
			let max_offset = total_stops.saturating_sub(visible_height);
			self.stop_scroll_offset = self.stop_scroll_offset.min(max_offset);
		}
	}
}
