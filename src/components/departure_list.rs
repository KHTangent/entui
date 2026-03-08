use ratatui::{
	layout::{Constraint, Layout, Margin},
	prelude::{Buffer, Rect},
	style::{Color, Style},
	widgets::{Block, Borders, Widget},
};

use crate::{
	components::departure_item::DepartureItem, entur_api_wrapper::departure_board::Departure,
	styles,
};

pub struct DepartureList<'a> {
	departures: &'a Vec<Departure>,
	selected_index: Option<usize>,
	focused: bool,
}

impl<'a> From<&'a Vec<Departure>> for DepartureList<'a> {
	fn from(value: &'a Vec<Departure>) -> Self {
		Self {
			departures: value,
			selected_index: None,
			focused: false,
		}
	}
}

impl<'a> DepartureList<'a> {
	pub fn with_selected_index(mut self, index: Option<usize>) -> Self {
		self.selected_index = index;
		self
	}

	pub fn with_focused(mut self, focused: bool) -> Self {
		self.focused = focused;
		self
	}
}

impl<'a> Widget for DepartureList<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let border_block = Block::default().borders(Borders::ALL).border_style(
			Style::new().fg(self
				.focused
				.then_some(styles::ACTIVE_COLOR)
				.unwrap_or(styles::INACTIVE_COLOR)),
		);
		border_block.render(area, buf);
		let inner_area = area.inner(Margin::new(1, 1));

		let num_departures = self.departures.len().max(inner_area.height as usize);
		let departure_list = Layout::vertical(vec![Constraint::Length(1); num_departures]);
		for (idx, (&area, departure)) in departure_list
			.split(inner_area)
			.iter()
			.zip(self.departures.into_iter())
			.enumerate()
		{
			let is_selected = self.selected_index == Some(idx);
			DepartureItem::from(departure)
				.with_line_color(Color::White, Color::Green)
				.with_selected(is_selected)
				.render(area, buf);
		}
	}
}
