use ratatui::{
	layout::{Constraint, Layout, Margin},
	prelude::{Buffer, Rect},
	style::Style,
	widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::{components::quay_item::QuayItem, entur_api_wrapper::departure_board::Quay, styles};

pub struct QuayListState {
	quays: Vec<Quay>,
	selected_index: Option<usize>,
	scroll_offset: usize,
}

impl QuayListState {
	pub fn new() -> Self {
		Self {
			quays: Vec::new(),
			selected_index: None,
			scroll_offset: 0,
		}
	}

	pub fn set_quays(&mut self, quays: Vec<Quay>) {
		self.quays = quays;
		self.selected_index = (!self.quays.is_empty()).then_some(0);
		self.scroll_offset = 0;
	}

	pub fn select_next(&mut self) {
		if let Some(index) = self.selected_index {
			if index + 1 < self.quays.len() {
				self.selected_index = Some(index + 1);
			}
		} else if !self.quays.is_empty() {
			self.selected_index = Some(0);
		}
	}

	pub fn select_previous(&mut self) {
		if let Some(index) = self.selected_index {
			if index > 0 {
				self.selected_index = Some(index - 1);
			}
		} else if !self.quays.is_empty() {
			self.selected_index = Some(self.quays.len() - 1);
		}
	}

	pub fn selected_quay(&self) -> Option<&Quay> {
		self.selected_index.and_then(|idx| self.quays.get(idx))
	}

	pub fn len(&self) -> usize {
		self.quays.len()
	}

	fn adjust_scroll(&mut self, visible_height: usize) {
		if let Some(selected) = self.selected_index {
			if selected < self.scroll_offset {
				// Selected is above visible area, scroll up
				self.scroll_offset = selected;
			} else if selected >= self.scroll_offset + visible_height {
				// Selected is below visible area, scroll down
				self.scroll_offset = selected.saturating_sub(visible_height.saturating_sub(1));
			}

			// Ensure scroll offset doesn't go beyond bounds
			let max_offset = self.quays.len().saturating_sub(visible_height);
			self.scroll_offset = self.scroll_offset.min(max_offset);
		}
	}
}

impl Default for QuayListState {
	fn default() -> Self {
		Self::new()
	}
}

pub struct QuayList {
	focused: bool,
}

impl QuayList {
	pub fn new() -> Self {
		Self { focused: false }
	}

	pub fn with_focused(mut self, focused: bool) -> Self {
		self.focused = focused;
		self
	}
}

impl Default for QuayList {
	fn default() -> Self {
		Self::new()
	}
}

impl StatefulWidget for QuayList {
	type State = QuayListState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut QuayListState) {
		let border_block = Block::default()
			.borders(Borders::ALL)
			.border_style(Style::new().fg(if self.focused {
				styles::ACTIVE_COLOR
			} else {
				styles::INACTIVE_COLOR
			}))
			.title_bottom("Quays");
		border_block.render(area, buf);
		let inner_area = area.inner(Margin::new(1, 1));

		let visible_height = inner_area.height as usize;
		let total_quays = state.len();

		// Adjust scroll based on current selection and visible height
		state.adjust_scroll(visible_height);

		// Calculate visible range based on scroll offset
		let start_index = state.scroll_offset.min(total_quays);
		let end_index = (start_index + visible_height).min(total_quays);
		let visible_count = end_index.saturating_sub(start_index);

		if visible_count == 0 {
			return;
		}

		let quay_list = Layout::vertical(vec![Constraint::Length(1); visible_count]);
		let areas = quay_list.split(inner_area);

		for (index, (&area, quay)) in areas
			.iter()
			.zip(state.quays[start_index..end_index].iter())
			.enumerate()
		{
			let absolute_index = start_index + index;
			let is_selected = state.selected_index == Some(absolute_index);
			QuayItem::from(quay)
				.with_selected(is_selected)
				.render(area, buf);
		}
	}
}
