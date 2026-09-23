use ratatui::{
	prelude::{Buffer, Rect},
	widgets::{Paragraph, Widget},
};

use crate::{components::list::render_selection, entur_api_wrapper::departure_board::Quay};

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
		render_selection(area, buf, self.is_selected);
		Paragraph::new(self.quay.to_label()).render(area, buf);
	}
}
