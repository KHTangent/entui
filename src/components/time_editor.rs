use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveTime, TimeZone, Timelike};
use ratatui::{
	layout::{Constraint, Layout, Margin, Rect},
	prelude::Buffer,
	style::Style,
	text::Line,
	widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};
use tui_input::backend::crossterm::EventHandler;

use crate::{components::list::Selectable, styles};

const FIELD_COUNT: usize = 5;
const FIELD_LABELS: [&str; FIELD_COUNT] = ["Year", "Month", "Day", "Hour", "Minute"];
const FIELD_WIDTHS: [u16; FIELD_COUNT] = [6; FIELD_COUNT];
const FIELD_GAP: u16 = 2;

const fn fields_width() -> u16 {
	let mut total = 0;
	let mut index = 0;
	while index < FIELD_COUNT {
		total += FIELD_WIDTHS[index];
		if index + 1 < FIELD_COUNT {
			total += FIELD_GAP;
		}
		index += 1;
	}
	total
}

const FIELDS_WIDTH: u16 = fields_width();
pub const EDITOR_WIDTH: u16 = FIELDS_WIDTH + 10;
pub const EDITOR_HEIGHT: u16 = 6;

pub struct TimeEditorState {
	fields: [tui_input::Input; FIELD_COUNT],
	focused: usize,
}

impl TimeEditorState {
	pub fn new() -> Self {
		let mut state = Self {
			fields: std::array::from_fn(|_| tui_input::Input::default()),
			focused: 0,
		};
		state.reset();
		state
	}

	pub fn reset(&mut self) {
		let now = Local::now();
		let values = [
			format!("{:04}", now.year()),
			format!("{:02}", now.month()),
			format!("{:02}", now.day()),
			format!("{:02}", now.hour()),
			format!("{:02}", now.minute()),
		];
		for (field, value) in self.fields.iter_mut().zip(values) {
			*field = tui_input::Input::new(value);
		}
		self.focused = 0;
	}

	pub fn handle_event(&mut self, event: &ratatui::crossterm::event::Event) {
		self.fields[self.focused].handle_event(event);
	}

	pub fn next_field(&mut self) {
		self.focused = (self.focused + 1) % FIELD_COUNT;
	}

	pub fn previous_field(&mut self) {
		self.focused = (self.focused + FIELD_COUNT - 1) % FIELD_COUNT;
	}

	pub fn parse(&self) -> Result<DateTime<Local>, String> {
		let year = self.parse_field(0)? as i32;
		let month = self.parse_field(1)?;
		let day = self.parse_field(2)?;
		let hour = self.parse_field(3)?;
		let minute = self.parse_field(4)?;

		let date = NaiveDate::from_ymd_opt(year, month, day)
			.ok_or_else(|| format!("invalid date: {year:04}-{month:02}-{day:02}"))?;
		let time = NaiveTime::from_hms_opt(hour, minute, 0)
			.ok_or_else(|| format!("invalid time: {hour:02}:{minute:02}"))?;
		let datetime = Local
			.from_local_datetime(&date.and_time(time))
			.single()
			.ok_or_else(|| "ambiguous or invalid local time".to_string())?;

		// Accept up to 60 seconds old timestamps to allow user to choose current minute
		if datetime < Local::now() - chrono::Duration::seconds(60) {
			return Err("time must be in the future".to_string());
		}

		Ok(datetime)
	}

	fn parse_field(&self, index: usize) -> Result<u32, String> {
		self.fields[index]
			.value()
			.trim()
			.parse::<u32>()
			.map_err(|_| format!("{} must be a number", FIELD_LABELS[index]))
	}

	fn focused_cursor(&self) -> u16 {
		self.fields[self.focused].visual_cursor() as u16
	}
}

impl Default for TimeEditorState {
	fn default() -> Self {
		Self::new()
	}
}

impl Selectable for TimeEditorState {
	fn select_next(&mut self) {
		self.next_field();
	}

	fn select_previous(&mut self) {
		self.previous_field();
	}
}

pub struct TimeEditor;

impl StatefulWidget for TimeEditor {
	type State = TimeEditorState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut TimeEditorState) {
		let block = Block::default()
			.borders(Borders::ALL)
			.border_style(Style::new().fg(styles::ACTIVE_COLOR))
			.title_top("Departure time")
			.title_bottom(Line::from("<Tab> next  <Enter> set  <Esc> cancel").centered());
		block.render(area, buf);

		let rows = content_rows(area);
		let labels = field_areas(fields_area(rows[0]));
		let inputs = field_areas(fields_area(rows[1]));

		for index in 0..FIELD_COUNT {
			let color = if index == state.focused {
				styles::ACTIVE_COLOR
			} else {
				styles::INACTIVE_COLOR
			};
			Paragraph::new(FIELD_LABELS[index])
				.style(Style::new().fg(color))
				.centered()
				.render(labels[index], buf);
			Paragraph::new(state.fields[index].value())
				.block(
					Block::default()
						.borders(Borders::ALL)
						.border_style(Style::new().fg(color)),
				)
				.render(inputs[index], buf);
		}
	}
}

pub fn cursor_position(area: Rect, state: &TimeEditorState) -> (u16, u16) {
	let rows = content_rows(area);
	let inputs = field_areas(fields_area(rows[1]));
	let field = inputs[state.focused];
	(field.x + 1 + state.focused_cursor(), field.y + 1)
}

fn content_rows(area: Rect) -> std::rc::Rc<[Rect]> {
	let inner = area.inner(Margin::new(1, 1));
	Layout::vertical([Constraint::Length(1), Constraint::Length(3)]).split(inner)
}

fn fields_area(area: Rect) -> Rect {
	let width = FIELDS_WIDTH.min(area.width);
	Rect {
		x: area.x + area.width.saturating_sub(width) / 2,
		width,
		height: area.height,
		..area
	}
}

fn field_areas(area: Rect) -> Vec<Rect> {
	let mut constraints = Vec::new();
	for (index, width) in FIELD_WIDTHS.iter().enumerate() {
		if index > 0 {
			constraints.push(Constraint::Length(FIELD_GAP));
		}
		constraints.push(Constraint::Length(*width));
	}
	Layout::horizontal(constraints)
		.split(area)
		.iter()
		.step_by(2)
		.copied()
		.collect()
}
