use std::ops::Range;

use ratatui::{
	layout::{Constraint, Layout, Margin},
	prelude::{Buffer, Rect},
	style::Style,
	widgets::{Block, Borders, Widget},
};

use crate::styles;

pub struct ListState<T> {
	items: Vec<T>,
	selected_index: Option<usize>,
	scroll_offset: usize,
}

impl<T> ListState<T> {
	pub fn new() -> Self {
		Self {
			items: Vec::new(),
			selected_index: None,
			scroll_offset: 0,
		}
	}

	pub fn set_items(&mut self, items: Vec<T>) {
		self.selected_index = (!items.is_empty()).then_some(0);
		self.items = items;
		self.scroll_offset = 0;
	}

	pub fn set_selected_index(&mut self, index: Option<usize>) {
		self.selected_index = index;
		self.scroll_offset = 0;
	}

	pub fn clear(&mut self) {
		self.items.clear();
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn select_next(&mut self) {
		if let Some(index) = self.selected_index {
			if index + 1 < self.items.len() {
				self.selected_index = Some(index + 1);
			}
		} else if !self.items.is_empty() {
			self.selected_index = Some(0);
		}
	}

	pub fn select_previous(&mut self) {
		if let Some(index) = self.selected_index {
			if index > 0 {
				self.selected_index = Some(index - 1);
			}
		} else if !self.items.is_empty() {
			self.selected_index = Some(self.items.len() - 1);
		}
	}

	pub fn deselect(&mut self) {
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn selected(&self) -> Option<&T> {
		self.selected_index.and_then(|idx| self.items.get(idx))
	}

	pub fn selected_index(&self) -> Option<usize> {
		self.selected_index
	}

	pub fn items(&self) -> &[T] {
		&self.items
	}

	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}

	pub fn len(&self) -> usize {
		self.items.len()
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
			let max_offset = self.items.len().saturating_sub(visible_height);
			self.scroll_offset = self.scroll_offset.min(max_offset);
		}
	}

	pub fn visible_range(&mut self, visible_height: usize) -> Range<usize> {
		self.adjust_scroll(visible_height);
		let start = self.scroll_offset.min(self.items.len());
		let end = (start + visible_height).min(self.items.len());
		start..end
	}
}

impl<T> Default for ListState<T> {
	fn default() -> Self {
		Self::new()
	}
}

pub fn render_selection(area: Rect, buf: &mut Buffer, selected: bool) {
	if selected {
		Block::new()
			.style(Style::new().bg(styles::SELECTION_BG))
			.render(area, buf);
	}
}

pub fn render_list<T>(
	area: Rect,
	buf: &mut Buffer,
	state: &mut ListState<T>,
	focused: bool,
	title: Option<&str>,
	row_height: u16,
	mut render_row: impl FnMut(&T, Rect, &mut Buffer, bool),
) {
	let mut block = Block::default()
		.borders(Borders::ALL)
		.border_style(Style::new().fg(if focused {
			styles::ACTIVE_COLOR
		} else {
			styles::INACTIVE_COLOR
		}));
	if let Some(title) = title {
		block = block.title_bottom(title);
	}
	block.render(area, buf);
	let inner_area = area.inner(Margin::new(1, 1));

	if row_height == 0 {
		return;
	}

	let visible_height = (inner_area.height as usize) / row_height as usize;

	if visible_height == 0 {
		return;
	}

	let range = state.visible_range(visible_height);

	if range.is_empty() {
		return;
	}

	let areas =
		Layout::vertical(vec![Constraint::Length(row_height); range.len()]).split(inner_area);
	let selected_index = state.selected_index();

	for (index, (&area, item)) in areas
		.iter()
		.zip(state.items()[range.clone()].iter())
		.enumerate()
	{
		let absolute_index = range.start + index;
		render_row(item, area, buf, selected_index == Some(absolute_index));
	}
}
