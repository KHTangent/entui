use chrono::Local;
use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	widgets::{Paragraph, Widget},
};

use crate::{entur_api_wrapper::departure_board::Stop, utils::format_relative_time};

pub struct StopItem<'a> {
	stop: &'a Stop,
}
impl<'a> From<&'a Stop> for StopItem<'a> {
	fn from(value: &'a Stop) -> Self {
		StopItem { stop: value }
	}
}

impl<'a> Widget for StopItem<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let line_layout = Layout::horizontal([Constraint::Fill(8), Constraint::Fill(2)]);
		let [name_box, time_box] = area.layout(&line_layout);

		Paragraph::new(self.stop.name.as_str()).render(name_box, buf);
		Paragraph::new(format_relative_time(&Local::now(), &self.stop.time))
			.centered()
			.render(time_box, buf);
	}
}
