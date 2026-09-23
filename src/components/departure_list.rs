use std::collections::HashMap;

use ratatui::{
	prelude::{Buffer, Rect},
	style::Color,
	widgets::{StatefulWidget, Widget},
};

use crate::{
	components::{
		departure_item::{DepartureItem, ROW_HEIGHT},
		list::{ListState, render_list},
	},
	entur_api_wrapper::departure_board::{Departure, Quay},
};

pub struct DepartureListState {
	list: ListState<Departure>,
	all_departures: Vec<Departure>,
	quays: HashMap<String, Quay>,
	quay_filter: Option<String>,
}

impl DepartureListState {
	pub fn new() -> Self {
		Self {
			list: ListState::new(),
			all_departures: Vec::new(),
			quays: HashMap::new(),
			quay_filter: None,
		}
	}

	pub fn set_departures(&mut self, departures: Vec<Departure>) {
		self.all_departures = departures;
		self.quay_filter = None;
		self.apply_filter();
	}

	pub fn set_quays(&mut self, quays: &[Quay]) {
		self.quays = quays
			.iter()
			.map(|quay| (quay.id.clone(), quay.clone()))
			.collect();
	}

	pub fn set_quay_filter(&mut self, quay_id: Option<&str>) {
		self.quay_filter = quay_id.map(str::to_string);
		self.apply_filter();
	}

	pub fn clear_quay_filter(&mut self) {
		self.set_quay_filter(None);
	}

	fn apply_filter(&mut self) {
		let departures = match &self.quay_filter {
			Some(quay_id) => self
				.all_departures
				.iter()
				.filter(|departure| departure.quay_id == *quay_id)
				.cloned()
				.collect(),
			None => self.all_departures.clone(),
		};
		self.list.set_items(departures);
	}

	pub fn select_next(&mut self) {
		self.list.select_next();
	}

	pub fn select_previous(&mut self) {
		self.list.select_previous();
	}

	pub fn deselect(&mut self) {
		self.list.deselect();
	}

	pub fn selected_departure(&self) -> Option<&Departure> {
		self.list.selected()
	}

	pub fn is_empty(&self) -> bool {
		self.list.is_empty()
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
		let DepartureListState { list, quays, .. } = state;
		render_list(
			area,
			buf,
			list,
			self.focused,
			None,
			ROW_HEIGHT,
			|departure, area, buf, selected| {
				let mut item = DepartureItem::from(departure)
					.with_line_color(Color::White, Color::Green)
					.with_selected(selected);
				if let Some(quay) = quays.get(&departure.quay_id) {
					item = item.with_quay_info(quay);
				}
				item.render(area, buf);
			},
		);
	}
}
