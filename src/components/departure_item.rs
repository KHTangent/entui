use chrono::{Timelike, Utc};
use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	style::{Color, Style, Stylize},
	widgets::{Paragraph, Widget},
};

use crate::entur_api_wrapper::departure_board::Departure;

pub struct DepartureItem {
	departure: Departure,
	line_color: Color,
	line_color_bg: Color,
}
impl From<Departure> for DepartureItem {
	fn from(value: Departure) -> Self {
		DepartureItem {
			departure: value,
			line_color: Color::default(),
			line_color_bg: Color::default(),
		}
	}
}

impl DepartureItem {
	pub fn with_line_color(mut self, text: Color, bg: Color) -> DepartureItem {
		self.line_color = text;
		self.line_color_bg = bg;
		self
	}
}

impl Widget for DepartureItem {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let line_layout = Layout::horizontal([
			Constraint::Fill(2),
			Constraint::Fill(8),
			Constraint::Fill(2),
		]);
		let [line_box, destination_box, time_box] = area.layout(&line_layout);
		Paragraph::new(self.departure.line)
			.bold()
			.centered()
			.style(Style::new().fg(self.line_color).bg(self.line_color_bg))
			.render(line_box, buf);
		Paragraph::new(self.departure.destination).render(destination_box, buf);
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
