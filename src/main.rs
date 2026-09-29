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
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()
		.unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
	tracing_subscriber::fmt()
		.with_env_filter(filter)
		.with_writer(file)
		.init();
	tracing::info!("Logging to {path}");
}

#[tokio::main]
async fn main() -> Result<()> {
	init_logging();
	color_eyre::install()?;
	tracing::info!(
		"Starting {} v{}",
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION")
	);
	let mut app = App::new();
	let mut terminal = ratatui::init();
	let result = app.run(&mut terminal).await.context("failed to run app");
	match &result {
		Ok(()) => tracing::info!("Shutting down"),
		Err(error) => tracing::error!("Application error: {error:?}"),
	}
	ratatui::restore();
	result
}
