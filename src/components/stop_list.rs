use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	widgets::Widget,
};

use crate::{components::stop_item::StopItem, entur_api_wrapper::departure_board::Stop};

pub struct StopList<'a> {
	stops: &'a Vec<Stop>,
}

impl<'a> From<&'a Vec<Stop>> for StopList<'a> {
	fn from(value: &'a Vec<Stop>) -> Self {
		Self { stops: value }
	}
}

impl<'a> Widget for StopList<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let num_departures = self.stops.len().max(area.height as usize);
		let stop_list = Layout::vertical(vec![Constraint::Length(1); num_departures]);
		for (&area, departure) in stop_list.split(area).iter().zip(self.stops.into_iter()) {
			StopItem::from(departure).render(area, buf);
		}
	}
}
