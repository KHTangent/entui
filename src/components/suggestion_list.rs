use ratatui::{
	prelude::{Buffer, Rect},
	widgets::{StatefulWidget, Widget},
};

use crate::{
	components::{
		list::{ListState, render_list},
		suggestion_item::SuggestionItem,
	},
	entur_api_wrapper::stop_register::StopSearchResult,
};

pub type SuggestionListState = ListState<StopSearchResult>;

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
		render_list(
			area,
			buf,
			state,
			self.focused,
			None,
			1,
			|suggestion, area, buf, selected| {
				SuggestionItem::from(suggestion)
					.with_selected(selected)
					.render(area, buf);
			},
		);
	}
}
