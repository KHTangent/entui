use chrono::Local;
use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	style::{Color, Style, Stylize},
	widgets::{Block, Paragraph, Widget},
};

use crate::{entur_api_wrapper::departure_board::Departure, utils::format_relative_time};

pub const ROW_HEIGHT: u16 = 3;
const LINE_BADGE_WIDTH: u16 = 7;

pub struct DepartureItem<'a> {
	departure: &'a Departure,
	line_color: Color,
	line_color_bg: Color,
	is_selected: bool,
}
impl<'a> From<&'a Departure> for DepartureItem<'a> {
	fn from(value: &'a Departure) -> Self {
		DepartureItem {
			departure: value,
			line_color: Color::default(),
			line_color_bg: Color::default(),
			is_selected: false,
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
}

impl<'a> Widget for DepartureItem<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		if self.is_selected {
			Block::new()
				.style(Style::new().bg(Color::DarkGray))
				.render(area, buf);
		}

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

		let text_area = Rect {
			y: text_box.y + 1,
			height: 1,
			..text_box
		};
		let [destination_box, time_box] = text_area.layout(&Layout::horizontal([
			Constraint::Fill(8),
			Constraint::Fill(2),
		]));
		Paragraph::new(self.departure.destination.as_str()).render(destination_box, buf);
		Paragraph::new(format_relative_time(&Local::now(), &self.departure.time))
			.render(time_box, buf);
	}
}
