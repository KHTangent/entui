use ratatui::{
	layout::{Constraint, Layout, Margin},
	prelude::{Buffer, Rect},
	style::Style,
	widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::styles;

const VISIBLE_SUGGESTIONS: usize = 10;

pub struct SuggestionListState {
	suggestions: Vec<String>,
}

impl SuggestionListState {
	pub fn new() -> Self {
		Self {
			suggestions: Vec::new(),
		}
	}

	pub fn set_suggestions(&mut self, suggestions: Vec<String>) {
		self.suggestions = suggestions;
	}

	pub fn len(&self) -> usize {
		self.suggestions.len()
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

		let visible_count = state.suggestions.len().min(VISIBLE_SUGGESTIONS);
		let suggestion_layout = Layout::vertical(vec![Constraint::Length(1); visible_count]);
		let areas = suggestion_layout.split(inner_area);

		for (&area, suggestion) in areas.iter().zip(state.suggestions[..visible_count].iter()) {
			Paragraph::new(suggestion.as_str()).render(area, buf);
		}
	}
}
