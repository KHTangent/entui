use ratatui::{
	layout::{Constraint, Layout, Margin},
	prelude::{Buffer, Rect},
	style::{Color, Style},
	widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::{
	components::departure_item::DepartureItem, entur_api_wrapper::departure_board::Departure,
	styles,
};

pub struct DepartureListState {
	departures: Vec<Departure>,
	selected_index: Option<usize>,
	scroll_offset: usize,
}

impl DepartureListState {
	pub fn new() -> Self {
		Self {
			departures: Vec::new(),
			selected_index: None,
			scroll_offset: 0,
		}
	}

	pub fn set_departures(&mut self, departures: Vec<Departure>) {
		self.departures = departures;
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn set_selected_index(&mut self, index: Option<usize>) {
		self.selected_index = index;
		self.scroll_offset = 0;
	}

	pub fn clear(&mut self) {
		self.departures.clear();
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn select_next(&mut self) {
		if let Some(index) = self.selected_index {
			if index + 1 < self.departures.len() {
				self.selected_index = Some(index + 1);
			}
		} else if !self.departures.is_empty() {
			self.selected_index = Some(0);
		}
	}

	pub fn select_previous(&mut self) {
		if let Some(index) = self.selected_index {
			if index > 0 {
				self.selected_index = Some(index - 1);
			}
		} else if !self.departures.is_empty() {
			self.selected_index = Some(self.departures.len() - 1);
		}
	}

	pub fn deselect(&mut self) {
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn selected_departure(&self) -> Option<&Departure> {
		self.selected_index.and_then(|idx| self.departures.get(idx))
	}

	pub fn is_empty(&self) -> bool {
		self.departures.is_empty()
	}

	pub fn len(&self) -> usize {
		self.departures.len()
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
			let max_offset = self.departures.len().saturating_sub(visible_height);
			self.scroll_offset = self.scroll_offset.min(max_offset);
		}
	}
}

impl Default for DepartureListState {
	fn default() -> Self {
		Self::new()
	}
}

pub struct DepartureList {
	focused: bool,
}

impl DepartureList {
	pub fn new() -> Self {
		Self { focused: false }
	}

	pub fn with_focused(mut self, focused: bool) -> Self {
		self.focused = focused;
		self
	}
}

impl Default for DepartureList {
	fn default() -> Self {
		Self::new()
	}
}

impl StatefulWidget for DepartureList {
	type State = DepartureListState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut DepartureListState) {
		let border_block = Block::default().borders(Borders::ALL).border_style(
			Style::new().fg(self
				.focused
				.then_some(styles::ACTIVE_COLOR)
				.unwrap_or(styles::INACTIVE_COLOR)),
		);
		border_block.render(area, buf);
		let inner_area = area.inner(Margin::new(1, 1));

		let visible_height = inner_area.height as usize;
		let total_departures = state.len();

		// Adjust scroll based on current selection and visible height
		state.adjust_scroll(visible_height);

		// Calculate visible range based on scroll offset
		let start_index = state.scroll_offset.min(total_departures);
		let end_index = (start_index + visible_height).min(total_departures);
		let visible_count = end_index.saturating_sub(start_index);

		if visible_count == 0 {
			return;
		}

		let departure_list = Layout::vertical(vec![Constraint::Length(1); visible_count]);
		let areas = departure_list.split(inner_area);

		for (index, (&area, departure)) in areas
			.iter()
			.zip(state.departures[start_index..end_index].iter())
			.enumerate()
		{
			let absolute_index = start_index + index;
			let is_selected = state.selected_index == Some(absolute_index);
			DepartureItem::from(departure)
				.with_line_color(Color::White, Color::Green)
				.with_selected(is_selected)
				.render(area, buf);
		}
	}
}
