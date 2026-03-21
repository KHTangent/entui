use std::{pin::Pin, time::Duration};

use crossterm::event::{Event as CrosstermEvent, EventStream};
use tokio::time::interval;
use tokio_stream::{Stream, StreamExt, StreamMap, wrappers::IntervalStream};

const RENDERS_PER_SECOND: f64 = 30.0;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum StreamName {
	Render,
	Crossterm,
}

#[derive(Clone, Debug)]
pub enum Event {
	Error,
	Render,
	Crossterm(CrosstermEvent),
}

pub struct Events {
	streams: StreamMap<StreamName, Pin<Box<dyn Stream<Item = Event>>>>,
}

impl Events {
	pub fn new() -> Self {
		Self {
			streams: StreamMap::from_iter([
				(StreamName::Render, make_render_stream()),
				(StreamName::Crossterm, make_crossterm_stream()),
			]),
		}
	}

	pub async fn next(&mut self) -> Option<Event> {
		self.streams.next().await.map(|(_name, event)| event)
	}
}

fn make_render_stream() -> Pin<Box<dyn Stream<Item = Event>>> {
	let render_delay = Duration::from_secs_f64(1.0 / RENDERS_PER_SECOND);
	let render_interval = interval(render_delay);
	Box::pin(IntervalStream::new(render_interval).map(|_| Event::Render))
}

fn make_crossterm_stream() -> Pin<Box<dyn Stream<Item = Event>>> {
	Box::pin(EventStream::new().fuse().filter_map(|event| match event {
		Ok(event) => Some(Event::Crossterm(event)),
		Err(_) => Some(Event::Error),
	}))
}
