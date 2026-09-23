use ratatui::{
	prelude::{Buffer, Rect},
	widgets::{StatefulWidget, Widget},
};

use crate::{
	components::{
		list::{ListState, render_list},
		quay_item::QuayItem,
	},
	entur_api_wrapper::departure_board::Quay,
};

pub type QuayListState = ListState<Quay>;

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
		render_list(
			area,
			buf,
			state,
			self.focused,
			Some("Quays"),
			1,
			|quay, area, buf, selected| {
				QuayItem::from(quay)
					.with_selected(selected)
					.render(area, buf);
			},
		);
	}
}
