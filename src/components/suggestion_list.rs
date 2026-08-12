use ratatui::{
	layout::{Constraint, Layout, Margin},
	prelude::{Buffer, Rect},
	style::{Color, Style},
	widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::styles;

pub struct SuggestionListState {
	suggestions: Vec<String>,
	selected_index: Option<usize>,
	scroll_offset: usize,
}

impl SuggestionListState {
	pub fn new() -> Self {
		Self {
			suggestions: Vec::new(),
			selected_index: None,
			scroll_offset: 0,
		}
	}

	pub fn set_suggestions(&mut self, suggestions: Vec<String>) {
		self.suggestions = suggestions;
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn select_next(&mut self) {
		if let Some(index) = self.selected_index {
			if index + 1 < self.suggestions.len() {
				self.selected_index = Some(index + 1);
			}
		} else if !self.suggestions.is_empty() {
			self.selected_index = Some(0);
		}
	}

	pub fn select_previous(&mut self) {
		if let Some(index) = self.selected_index {
			if index > 0 {
				self.selected_index = Some(index - 1);
			}
		} else if !self.suggestions.is_empty() {
			self.selected_index = Some(self.suggestions.len() - 1);
		}
	}

	pub fn deselect(&mut self) {
		self.selected_index = None;
		self.scroll_offset = 0;
	}

	pub fn selected_suggestion(&self) -> Option<&String> {
		self.selected_index
			.and_then(|idx| self.suggestions.get(idx))
	}

	pub fn len(&self) -> usize {
		self.suggestions.len()
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
			let max_offset = self.suggestions.len().saturating_sub(visible_height);
			self.scroll_offset = self.scroll_offset.min(max_offset);
		}
	}
}

impl Default for SuggestionListState {
	fn default() -> Self {
		Self::new()
	}
}

pub struct SuggestionList {
	focused: bool,
}

impl SuggestionList {
	pub fn new() -> Self {
		Self { focused: false }
	}

	pub fn with_focused(mut self, focused: bool) -> Self {
		self.focused = focused;
		self
	}
}

impl Default for SuggestionList {
	fn default() -> Self {
		Self::new()
	}
}

impl StatefulWidget for SuggestionList {
	type State = SuggestionListState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut SuggestionListState) {
		let border_block = Block::default().borders(Borders::ALL).border_style(
			Style::new().fg(self
				.focused
				.then_some(styles::ACTIVE_COLOR)
				.unwrap_or(styles::INACTIVE_COLOR)),
		);
		border_block.render(area, buf);
		let inner_area = area.inner(Margin::new(1, 1));

		let visible_height = inner_area.height as usize;
		let total_suggestions = state.len();

		// Adjust scroll based on current selection and visible height
		state.adjust_scroll(visible_height);

		// Calculate visible range based on scroll offset
		let start_index = state.scroll_offset.min(total_suggestions);
		let end_index = (start_index + visible_height).min(total_suggestions);
		let visible_count = end_index.saturating_sub(start_index);

		if visible_count == 0 {
			return;
		}

		let suggestion_layout = Layout::vertical(vec![Constraint::Length(1); visible_count]);
		let areas = suggestion_layout.split(inner_area);

		for (index, (&area, suggestion)) in areas
			.iter()
			.zip(state.suggestions[start_index..end_index].iter())
			.enumerate()
		{
			let absolute_index = start_index + index;
			if state.selected_index == Some(absolute_index) {
				Block::new()
					.style(Style::new().bg(Color::DarkGray))
					.render(area, buf);
			}
			Paragraph::new(suggestion.as_str()).render(area, buf);
		}
	}
}
