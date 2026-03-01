use std::time::Duration;

use color_eyre::Result;
use color_eyre::eyre::Context;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Margin};
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders};
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
	let main_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
	let [departure_board, _details] = frame.area().layout(&main_layout);
	let block = Block::default().borders(Borders::ALL);
	frame.render_widget(block, departure_board);
	let departure_board = departure_board.inner(Margin::new(1, 1));

	let departures = get_departures("Siemens");
	let departure_list = Layout::vertical(vec![Constraint::Length(1); departures.len()]);
	for (&area, departure) in departure_list
		.split(departure_board)
		.iter()
		.zip(departures.into_iter())
	{
		frame.render_widget(
			DepartureItem::from(departure).with_line_color(Color::White, Color::Green),
			area,
		);
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
