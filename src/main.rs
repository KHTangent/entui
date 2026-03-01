use std::time::Duration;

use color_eyre::Result;
use color_eyre::eyre::Context;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Margin};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use crate::components::departure_list::DepartureList;
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
	let [main_layout_rect, search_bar_rect] = frame.area().layout(&Layout::vertical([
		Constraint::Fill(1),
		Constraint::Length(5),
	]));
	let [departures_rect, details_rect] = main_layout_rect.layout(&Layout::horizontal([
		Constraint::Fill(1),
		Constraint::Fill(1),
	]));

	let search_bar_block = Block::default()
		.borders(Borders::ALL)
		.title_bottom("Stop name");
	frame.render_widget(search_bar_block, search_bar_rect);
	let departure_board_block = Block::default().borders(Borders::ALL);
	frame.render_widget(departure_board_block, departures_rect);
	let details_block = Block::default().borders(Borders::ALL);
	frame.render_widget(details_block, details_rect);

	let search_inner = search_bar_rect.inner(Margin::new(2, 2));
	let dummy_search_text = Paragraph::new("Siemens");
	frame.render_widget(dummy_search_text, search_inner);

	let departure_board = departures_rect.inner(Margin::new(1, 1));
	let departures = get_departures("Siemens");
	frame.render_widget(DepartureList::from(&departures), departure_board);
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
