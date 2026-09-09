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

pub fn init_logging() {
	let Ok(path) = std::env::var("ENTUI_LOG") else {
		return;
	};
	let file =
		std::fs::File::create(&path).expect("failed to create log file specified by ENTUI_LOG");
	tracing_subscriber::fmt()
		.with_max_level(tracing::Level::INFO)
		.with_writer(file)
		.init();
}

#[tokio::main]
async fn main() -> Result<()> {
	init_logging();
	color_eyre::install()?;
	let mut app = App::new();
	let mut terminal = ratatui::init();
	let result = app.run(&mut terminal).await.context("failed to run app");
	ratatui::restore();
	result
}
