use chrono::{Timelike, Utc};
use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	style::{Color, Style, Stylize},
	widgets::{Block, Paragraph, Widget},
};

use crate::entur_api_wrapper::departure_board::Departure;

pub struct DepartureItem<'a> {
	departure: &'a Departure,
	line_color: Color,
	line_color_bg: Color,
	is_selected: bool,
}
impl<'a> From<&'a Departure> for DepartureItem<'a> {
	fn from(value: &'a Departure) -> Self {
		DepartureItem {
			departure: value,
			line_color: Color::default(),
			line_color_bg: Color::default(),
			is_selected: false,
		}
	}
}

impl<'a> DepartureItem<'a> {
	pub fn with_line_color(mut self, text: Color, bg: Color) -> DepartureItem<'a> {
		self.line_color = text;
		self.line_color_bg = bg;
		self
	}

	pub fn with_selected(mut self, selected: bool) -> DepartureItem<'a> {
		self.is_selected = selected;
		self
	}
}

impl<'a> Widget for DepartureItem<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let line_layout = Layout::horizontal([
			Constraint::Fill(2),
			Constraint::Fill(8),
			Constraint::Fill(2),
		]);
		let [line_box, destination_box, time_box] = area.layout(&line_layout);

		if self.is_selected {
			Block::new()
				.style(Style::new().bg(Color::DarkGray))
				.render(area, buf);
		}
		Paragraph::new(self.departure.line.as_str())
			.bold()
			.centered()
			.style(Style::new().fg(self.line_color).bg(self.line_color_bg))
			.render(line_box, buf);
		Paragraph::new(self.departure.destination.as_str()).render(destination_box, buf);
		let relative_time = match (Utc::now() - self.departure.time).num_minutes() {
			0 => "now".to_string(),
			n @ 1..=10 => format!("{n} minutes"),
			_ => format!(
				"{:02}:{:02}",
				self.departure.time.hour(),
				self.departure.time.minute()
			),
		};
		Paragraph::new(relative_time).render(time_box, buf);
	}
}
