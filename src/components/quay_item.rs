use ratatui::{
	prelude::{Buffer, Rect},
	style::{Color, Style},
	widgets::{Block, Paragraph, Widget},
};

use crate::entur_api_wrapper::departure_board::Quay;

pub struct QuayItem<'a> {
	quay: &'a Quay,
	is_selected: bool,
}
impl<'a> From<&'a Quay> for QuayItem<'a> {
	fn from(value: &'a Quay) -> Self {
		QuayItem {
			quay: value,
			is_selected: false,
		}
	}
}

impl<'a> QuayItem<'a> {
	pub fn with_selected(mut self, selected: bool) -> QuayItem<'a> {
		self.is_selected = selected;
		self
	}
}

impl<'a> Widget for QuayItem<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		if self.is_selected {
			Block::new()
				.style(Style::new().bg(Color::DarkGray))
				.render(area, buf);
		}
		Paragraph::new(self.quay.to_label()).render(area, buf);
	}
}
