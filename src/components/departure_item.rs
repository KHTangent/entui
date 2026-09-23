use chrono::Local;
use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	style::{Color, Style, Stylize},
	widgets::{Block, Paragraph, Widget},
};

use crate::{
	components::list::render_selection,
	entur_api_wrapper::departure_board::{Departure, Quay},
	utils::format_relative_time,
};

pub const ROW_HEIGHT: u16 = 3;
const LINE_BADGE_WIDTH: u16 = 7;

pub struct DepartureItem<'a> {
	departure: &'a Departure,
	line_color: Color,
	line_color_bg: Color,
	is_selected: bool,
	quay_info: Option<&'a Quay>,
}
impl<'a> From<&'a Departure> for DepartureItem<'a> {
	fn from(value: &'a Departure) -> Self {
		DepartureItem {
			departure: value,
			line_color: Color::default(),
			line_color_bg: Color::default(),
			is_selected: false,
			quay_info: None,
		}
	}
}

impl<'a> DepartureItem<'a> {
	pub fn with_line_color(mut self, text: Color, bg: Color) -> DepartureItem<'a> {
		self.line_color = text;
		self.line_color_bg = bg;
		self
	}

	pub fn with_selected(mut self, selected: bool) -> DepartureItem<'a> {
		self.is_selected = selected;
		self
	}

	pub fn with_quay_info(mut self, quay_info: &'a Quay) -> DepartureItem<'a> {
		self.quay_info = Some(quay_info);
		self
	}
}

impl<'a> Widget for DepartureItem<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		render_selection(area, buf, self.is_selected);

		let [line_box, text_box] = area.layout(&Layout::horizontal([
			Constraint::Length(LINE_BADGE_WIDTH),
			Constraint::Fill(1),
		]));

		Paragraph::new(self.departure.line.as_str())
			.bold()
			.centered()
			.style(Style::new().fg(self.line_color).bg(self.line_color_bg))
			.block(Block::bordered().border_style(Style::new().fg(self.line_color_bg)))
			.render(line_box, buf);

		let [destination_column, time_box] = text_box.layout(&Layout::horizontal([
			Constraint::Fill(8),
			Constraint::Fill(2),
		]));
		let [_, destination_box, label_box] = destination_column.layout(&Layout::vertical([
			Constraint::Length(1),
			Constraint::Length(1),
			Constraint::Length(1),
		]));
		Paragraph::new(self.departure.destination.as_str()).render(destination_box, buf);
		if let Some(quay) = self.quay_info {
			Paragraph::new(quay.to_label())
				.style(Style::new().dim())
				.render(label_box, buf);
		}
		let [_, time_row, _] = time_box.layout(&Layout::vertical([
			Constraint::Length(1),
			Constraint::Length(1),
			Constraint::Length(1),
		]));
		Paragraph::new(format_relative_time(&Local::now(), &self.departure.time))
			.render(time_row, buf);
	}
}
