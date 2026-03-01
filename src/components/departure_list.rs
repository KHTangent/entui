use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	style::Color,
	widgets::Widget,
};

use crate::{
	components::departure_item::DepartureItem, entur_api_wrapper::departure_board::Departure,
};

pub struct DepartureList<'a> {
	departures: &'a Vec<Departure>,
}

impl<'a> From<&'a Vec<Departure>> for DepartureList<'a> {
	fn from(value: &'a Vec<Departure>) -> Self {
		Self { departures: value }
	}
}

impl<'a> Widget for DepartureList<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let num_departures = self.departures.len().max(area.height as usize);
		let departure_list = Layout::vertical(vec![Constraint::Length(1); num_departures]);
		for (&area, departure) in departure_list
			.split(area)
			.iter()
			.zip(self.departures.into_iter())
		{
			DepartureItem::from(departure)
				.with_line_color(Color::White, Color::Green)
				.render(area, buf);
		}
	}
}
