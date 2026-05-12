use ratatui::{
	layout::{Constraint, Layout, Margin},
	prelude::{Buffer, Rect},
	style::Style,
	widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::{components::stop_item::StopItem, entur_api_wrapper::departure_board::Stop, styles};

pub struct StopListState {
	stops: Vec<Stop>,
	selected_index: Option<usize>,
	scroll_offset: usize,
}

impl StopListState {
	pub fn new() -> Self {
		Self {
			stops: Vec::new(),
			selected_index: None,
			scroll_offset: 0,
		}
	}

	pub fn set_stops(&mut self, stops: Vec<Stop>) {
		self.stops = stops;
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn set_selected_index(&mut self, index: Option<usize>) {
		self.selected_index = index;
		self.scroll_offset = 0;
	}

	pub fn clear(&mut self) {
		self.stops.clear();
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn select_next(&mut self) {
		if let Some(index) = self.selected_index {
			if index + 1 < self.stops.len() {
				self.selected_index = Some(index + 1);
			}
		} else if !self.stops.is_empty() {
			self.selected_index = Some(0);
		}
	}

	pub fn select_previous(&mut self) {
		if let Some(index) = self.selected_index {
			if index > 0 {
				self.selected_index = Some(index - 1);
			}
		} else if !self.stops.is_empty() {
			self.selected_index = Some(self.stops.len() - 1);
		}
	}

	pub fn deselect(&mut self) {
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn selected_stop(&self) -> Option<&Stop> {
		self.selected_index.and_then(|idx| self.stops.get(idx))
	}

	pub fn is_empty(&self) -> bool {
		self.stops.is_empty()
	}

	pub fn len(&self) -> usize {
		self.stops.len()
	}

	fn adjust_scroll(&mut self, visible_height: usize) {
		if let Some(selected) = self.selected_index {
			if selected < self.scroll_offset {
				// Selected is above visible area, scroll up
				self.scroll_offset = selected;
			} else if selected >= self.scroll_offset + visible_height {
				// Selected is below visible area, scroll down
				self.scroll_offset = selected.saturating_sub(visible_height - 1);
			}

			// Ensure scroll offset doesn't go beyond bounds
			let max_offset = self.stops.len().saturating_sub(visible_height);
			self.scroll_offset = self.scroll_offset.min(max_offset);
		}
	}
}

impl Default for StopListState {
	fn default() -> Self {
		Self::new()
	}
}

pub struct StopList {
	focused: bool,
}

impl StopList {
	pub fn new() -> Self {
		Self { focused: false }
	}

	pub fn with_focused(mut self, focused: bool) -> Self {
		self.focused = focused;
		self
	}
}

impl Default for StopList {
	fn default() -> Self {
		Self::new()
	}
}

impl StatefulWidget for StopList {
	type State = StopListState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut StopListState) {
		let border_block = Block::default().borders(Borders::ALL).border_style(
			Style::new().fg(self
				.focused
				.then_some(styles::ACTIVE_COLOR)
				.unwrap_or(styles::INACTIVE_COLOR)),
		);
		border_block.render(area, buf);
		let inner_area = area.inner(Margin::new(1, 1));

		let visible_height = inner_area.height as usize;
		let total_stops = state.len();

		// Adjust scroll based on current selection and visible height
		state.adjust_scroll(visible_height);

		// Calculate visible range based on scroll offset
		let start_index = state.scroll_offset.min(total_stops);
		let end_index = (start_index + visible_height).min(total_stops);
		let visible_count = end_index.saturating_sub(start_index);

		if visible_count == 0 {
			return;
		}

		let stop_list = Layout::vertical(vec![Constraint::Length(1); visible_count]);
		let areas = stop_list.split(inner_area);

		for (index, (&area, stop)) in areas
			.iter()
			.zip(state.stops[start_index..end_index].iter())
			.enumerate()
		{
			let absolute_index = start_index + index;
			let is_selected = state.selected_index == Some(absolute_index);
			StopItem::from(stop)
				.with_selected(is_selected)
				.render(area, buf);
		}
	}
}
