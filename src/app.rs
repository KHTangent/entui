use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Margin};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use crate::components::departure_list::DepartureList;
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
}

impl App {
	pub fn new() -> Self {
		Self {
			current_state: AppState::default(),
			active_departures: vec![],
		}
	}

	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
		loop {
			self.active_departures = get_departures("Siemens");

			terminal.draw(|frame| self.render(frame))?;

			if let Event::Key(key) = event::read()? {
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
						_ => {}
					},
					AppState::EditSearch => match key.code {
						KeyCode::Esc if key.kind == KeyEventKind::Press => {
							self.current_state = AppState::DepartureList;
						}
						_ => {}
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

		let search_bar_block = Block::default()
			.borders(Borders::ALL)
			.border_style(
				Style::new().fg(self.current_state.color_if_state_is(AppState::EditSearch)),
			)
			.title_bottom("Stop name");
		frame.render_widget(search_bar_block, search_bar_rect);
		let departure_board_block = Block::default().borders(Borders::ALL).border_style(
			Style::new().fg(self
				.current_state
				.color_if_state_is(AppState::DepartureList)),
		);
		frame.render_widget(departure_board_block, departures_rect);
		let details_block = Block::default().borders(Borders::ALL);
		frame.render_widget(details_block, details_rect);

		let search_inner = search_bar_rect.inner(Margin::new(2, 2));
		let dummy_search_text = Paragraph::new("Siemens");
		frame.render_widget(dummy_search_text, search_inner);

		let departure_board = departures_rect.inner(Margin::new(1, 1));
		frame.render_widget(
			DepartureList::from(&self.active_departures),
			departure_board,
		);
	}
}
