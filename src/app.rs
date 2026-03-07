use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Margin};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tui_input::backend::crossterm::EventHandler;

use crate::components::departure_list::DepartureList;
use crate::components::stop_list::StopList;
use crate::entur_api_wrapper::departure_board::{Departure, get_departures};

const ACTIVE_COLOR: Color = Color::Yellow;
const INACTIVE_COLOR: Color = Color::White;

#[derive(PartialEq, Eq, Default)]
enum AppState {
	#[default]
	DepartureList,
	EditSearch,
}
impl AppState {
	pub fn color_if_state_is(&self, other: AppState) -> Color {
		if *self == other {
			ACTIVE_COLOR
		} else {
			INACTIVE_COLOR
		}
	}
}

pub struct App {
	current_state: AppState,
	active_departures: Vec<Departure>,
	selected_departure_index: Option<usize>,
	stop_input: tui_input::Input,
}

impl App {
	pub fn new() -> Self {
		Self {
			current_state: AppState::default(),
			active_departures: vec![],
			stop_input: tui_input::Input::default(),
			selected_departure_index: None,
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

		let departure_board_block = Block::default().borders(Borders::ALL).border_style(
			Style::new().fg(self
				.current_state
				.color_if_state_is(AppState::DepartureList)),
		);
		frame.render_widget(departure_board_block, departures_rect);
		let details_block = Block::default().borders(Borders::ALL);
		frame.render_widget(details_block, details_rect);

		let search_text = Paragraph::new(self.stop_input.value()).block(
			Block::default()
				.borders(Borders::ALL)
				.padding(Padding::uniform(1))
				.border_style(
					Style::new().fg(self.current_state.color_if_state_is(AppState::EditSearch)),
				)
				.title_bottom("Stop name"),
		);
		frame.render_widget(search_text, search_bar_rect);
		if self.current_state == AppState::EditSearch {
			let x = self.stop_input.visual_cursor() as u16;
			frame.set_cursor_position((search_bar_rect.x + x + 2, search_bar_rect.y + 2 as u16));
		}

		let departure_board = departures_rect.inner(Margin::new(1, 1));
		frame.render_widget(
			DepartureList::from(&self.active_departures)
				.with_selected_index(self.selected_departure_index),
			departure_board,
		);

		if let Some(selected_departure) = self.selected_departure_index {
			let stops = self.active_departures[selected_departure].get_stops();
			let stops_board = details_rect.inner(Margin::new(1, 1));
			frame.render_widget(StopList::from(&stops), stops_board);
		}
	}

	fn select_next_departure(&mut self) {
		if let Some(idx) = self.selected_departure_index {
			if idx + 1 < self.active_departures.len() {
				self.selected_departure_index = Some(idx + 1);
			}
		} else if !self.active_departures.is_empty() {
			self.selected_departure_index = Some(0);
		}
	}

	fn select_previous_departure(&mut self) {
		if let Some(idx) = self.selected_departure_index {
			if idx > 0 {
				self.selected_departure_index = Some(idx - 1);
			}
		} else if !self.active_departures.is_empty() {
			self.selected_departure_index = Some(self.active_departures.len() - 1);
		}
	}

	fn deselect_departure(&mut self) {
		self.selected_departure_index = None;
	}
}
