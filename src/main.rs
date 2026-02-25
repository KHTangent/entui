use std::fmt::format;
use std::time::Duration;

use color_eyre::Result;
use color_eyre::eyre::Context;
use itertools::Itertools;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame};

use crate::entur_api_wrapper::departure_board::get_departures;

mod entur_api_wrapper;

fn main() -> Result<()> {
	color_eyre::install()?; // augment errors / panics with easy to read messages
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
	let departures: String = get_departures("Siemens")
		.iter()
		.map(|d| format!("{} {}   {}", d.line, d.destination, d.time.to_string()).to_string())
		.join("\n");
	let greeting = Paragraph::new(departures);
	frame.render_widget(greeting, frame.area());
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
