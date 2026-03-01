use std::time::Duration;

use color_eyre::Result;
use color_eyre::eyre::Context;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::{DefaultTerminal, Frame};

use crate::components::departure_item::DepartureItem;
use crate::entur_api_wrapper::departure_board::get_departures;

mod components;
mod entur_api_wrapper;

fn main() -> Result<()> {
	color_eyre::install()?;
	ratatui::run(run).context("failed to run app")
}
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
	loop {
		terminal.draw(render)?;
		if should_quit()? {
			break;
		}
	}
	Ok(())
}
fn render(frame: &mut Frame) {
	let departures = get_departures("Siemens");
	let list = Layout::vertical(vec![Constraint::Length(1); departures.len()]);
	for (&area, departure) in list.split(frame.area()).iter().zip(departures.into_iter()) {
		frame.render_widget(DepartureItem::from(departure), area);
	}
}

fn should_quit() -> Result<bool> {
	if event::poll(Duration::from_millis(250)).context("event poll failed")? {
		let q_pressed = event::read()
			.context("event read failed")?
			.as_key_press_event()
			.is_some_and(|key| key.code == KeyCode::Char('q'));
		return Ok(q_pressed);
	}
	Ok(false)
}
