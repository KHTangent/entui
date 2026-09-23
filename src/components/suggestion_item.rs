use ratatui::{
	prelude::{Buffer, Rect},
	widgets::{Paragraph, Widget},
};

use crate::{
	components::list::render_selection, entur_api_wrapper::stop_register::StopSearchResult,
};

pub struct SuggestionItem<'a> {
	suggestion: &'a StopSearchResult,
	is_selected: bool,
}
impl<'a> From<&'a StopSearchResult> for SuggestionItem<'a> {
	fn from(value: &'a StopSearchResult) -> Self {
		SuggestionItem {
			suggestion: value,
			is_selected: false,
		}
	}
}

impl<'a> SuggestionItem<'a> {
	pub fn with_selected(mut self, selected: bool) -> SuggestionItem<'a> {
		self.is_selected = selected;
		self
	}
}

impl<'a> Widget for SuggestionItem<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		render_selection(area, buf, self.is_selected);
		Paragraph::new(self.suggestion.label.as_str()).render(area, buf);
	}
}
