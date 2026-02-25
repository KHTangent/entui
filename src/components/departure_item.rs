use chrono::{Timelike, Utc};
use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	style::Stylize,
	widgets::{Paragraph, Widget},
};

use crate::entur_api_wrapper::departure_board::Departure;

pub struct DepartureItem {
	departure: Departure,
}
impl From<Departure> for DepartureItem {
	fn from(value: Departure) -> Self {
		DepartureItem { departure: value }
	}
}

impl Widget for DepartureItem {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let line_layout = Layout::horizontal([
			Constraint::Length(2),
			Constraint::Fill(1),
			Constraint::Length(2),
		]);
		let [line_box, destination_box, time_box] = area.layout(&line_layout);
		Paragraph::new(self.departure.line)
			.bold()
			.centered()
			.render(line_box, buf);
		Paragraph::new(self.departure.destination).render(destination_box, buf);
		let relative_time = match (Utc::now() - self.departure.time).num_minutes() {
			0 => "now".to_string(),
			n @ 1..=10 => format!("{n} minutes"),
			_ => format!(
				"{:0<2}:{:0<2}",
				self.departure.time.hour(),
				self.departure.time.minute()
			),
		};
		Paragraph::new(relative_time).render(time_box, buf);
	}
}
