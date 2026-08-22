use color_eyre::Result;
use color_eyre::eyre::Context;

use crate::app::App;

mod actions;
mod app;
mod components;
mod entur_api_wrapper;
mod events;
mod styles;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
	color_eyre::install()?;
	let mut app = App::new();
	let mut terminal = ratatui::init();
	let result = app.run(&mut terminal).await.context("failed to run app");
	ratatui::restore();
	result
}
