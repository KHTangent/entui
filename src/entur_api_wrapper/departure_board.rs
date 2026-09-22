use chrono::Local;

use crate::entur_api_wrapper::{api, error::ApiResult};

#[derive(Debug, Clone)]
pub struct Departure {
	pub quay_id: String,
	pub id: String,
	pub line: String,
	pub destination: String,
	pub time: chrono::DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct Stop {
	pub quay_id: String,
	pub name: String,
	pub time: chrono::DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct Quay {
	pub id: String,
	pub name: String,
	pub public_code: Option<String>,
	pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DepartureBoardData {
	pub departures: Vec<Departure>,
	pub quays: Vec<Quay>,
}

impl Departure {
	pub async fn get_stops(&self) -> ApiResult<Vec<Stop>> {
		let client = reqwest::Client::new();
		let result = api::JourneyPlanner::get_stops(&client, &self.id)
			.await?
			.data
			.service_journey
			.estimated_calls
			.into_iter()
			.map(|call| Stop {
				quay_id: call.quay.id,
				name: call.quay.name,
				time: call.expected_departure_time.parse().unwrap_or(Local::now()),
			})
			.collect();
		Ok(result)
	}
}

pub async fn get_departures(from: &str) -> ApiResult<DepartureBoardData> {
	let client = reqwest::Client::new();
	let stop_place = api::JourneyPlanner::get_departures(&client, from, 30)
		.await?
		.data
		.stop_place;

	let departures = stop_place
		.estimated_calls
		.into_iter()
		.map(|call| Departure {
			quay_id: call.quay.id,
			id: call.service_journey.id,
			destination: call.destination_display.front_text,
			line: call.service_journey.journey_pattern.line.public_code,
			time: call
				.expected_departure_time
				.parse()
				.unwrap_or_else(|_| Local::now()),
		})
		.collect();

	let quays = stop_place
		.quays
		.into_iter()
		.map(|quay| Quay {
			id: quay.id,
			name: quay.name,
			public_code: quay.public_code,
			description: quay.description,
		})
		.collect();

	Ok(DepartureBoardData { departures, quays })
}
