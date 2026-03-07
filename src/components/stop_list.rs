use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	widgets::Widget,
};

use crate::{components::stop_item::StopItem, entur_api_wrapper::departure_board::Stop};

pub struct StopList<'a> {
	stops: &'a Vec<Stop>,
	selected_index: Option<usize>,
	scroll_offset: usize,
}

impl<'a> From<&'a Vec<Stop>> for StopList<'a> {
	fn from(value: &'a Vec<Stop>) -> Self {
		Self {
			stops: value,
			selected_index: None,
			scroll_offset: 0,
		}
	}
}

impl<'a> StopList<'a> {
	pub fn with_selected_index(mut self, index: Option<usize>) -> Self {
		self.selected_index = index;
		self
	}

	pub fn with_scroll_offset(mut self, offset: usize) -> Self {
		self.scroll_offset = offset;
		self
	}
}

impl<'a> Widget for StopList<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let visible_count = area.height as usize;
		let total_stops = self.stops.len();

		// Calculate visible range based on scroll offset
		let start_index = self.scroll_offset.min(total_stops);
		let end_index = (start_index + visible_count).min(total_stops);
		let visible_stops = end_index.saturating_sub(start_index);

		if visible_stops == 0 {
			return;
		}

		let stop_list = Layout::vertical(vec![Constraint::Length(1); visible_stops]);
		let areas = stop_list.split(area);

		for (index, (&area, stop)) in areas
			.iter()
			.zip(self.stops[start_index..end_index].iter())
			.enumerate()
		{
			let absolute_index = start_index + index;
			let is_selected = self.selected_index == Some(absolute_index);
			StopItem::from(stop)
				.with_selected(is_selected)
				.render(area, buf);
		}
	}
}
