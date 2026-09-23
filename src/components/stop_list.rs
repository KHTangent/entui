use ratatui::{
	prelude::{Buffer, Rect},
	widgets::{StatefulWidget, Widget},
};

use crate::{
	components::{
		list::{ListState, render_list},
		stop_item::StopItem,
	},
	entur_api_wrapper::departure_board::Stop,
};

pub type StopListState = ListState<Stop>;

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
		render_list(
			area,
			buf,
			state,
			self.focused,
			None,
			1,
			|stop, area, buf, selected| {
				StopItem::from(stop)
					.with_selected(selected)
					.render(area, buf);
			},
		);
	}
}
