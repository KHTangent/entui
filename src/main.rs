use color_eyre::Result;
use color_eyre::eyre::Context;

use crate::app::App;

mod app;
mod components;
mod entur_api_wrapper;
mod utils;

fn main() -> Result<()> {
	color_eyre::install()?;
	let mut app = App::new();
	ratatui::run(|terminal| app.run(terminal)).context("failed to run app")
}
