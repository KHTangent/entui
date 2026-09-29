use std::collections::HashMap;
use std::time::Instant;

use chrono::{DateTime, Local};
use serde_json::json;
use tracing::{debug, info, warn};

use crate::entur_api_wrapper::raw_types::{
	geocoding::AutocompleteResponse,
	journey_planner::{DepartureBoard, RequestQuery, StopList},
};

const GEOCODER_URL: &str = "https://api.entur.io/geocoder/v3/autocomplete";
const JOURNEYPLANNER_URL: &str = "https://api.entur.io/journey-planner/v3/graphql";
const CLIENT_NAME: &str = "KHTangent-Entui";

const DEPARTUREBOARD_QUERY: &str = r#"
query departureBoard($id: String!, $departures: Int!, $startTime: DateTime!) {
	stopPlace(id: $id) {
		id
		name
		quays {
			id
			name
			publicCode
			description
		}
		estimatedCalls(timeRange: 72100, numberOfDepartures: $departures, startTime: $startTime) {
			realtime
			aimedDepartureTime
			expectedDepartureTime
			forBoarding
			destinationDisplay {
				frontText
			}
			quay {
				id
				name
			}
			serviceJourney {
				id
				journeyPattern {
					line {
						id
						publicCode
						name
						transportMode
					}
				}
			}
		}
	}
}
"#;

const STOPS_QUERY: &str = r#"
query stopLists($id: String!) {
	serviceJourney(id: $id) {
		estimatedCalls {
			quay {
				name
				id
			}
			expectedDepartureTime
		}
	}
}
"#;

pub struct Geocoder;

impl Geocoder {
	pub async fn autocomplete(
		client: &reqwest::Client,
		query: &str,
	) -> Result<AutocompleteResponse, reqwest::Error> {
		info!(r#"Requesting search data for "{}""#, query);
		let started = Instant::now();
		let response = client
			.get(GEOCODER_URL)
			.query(&[("layers", "stopPlace"), ("q", query)])
			.header("ET-Client-Name", CLIENT_NAME)
			.send()
			.await?;
		log_response("Search data", query, &response, started);
		response.json::<AutocompleteResponse>().await
	}
}

pub struct JourneyPlanner;

impl JourneyPlanner {
	pub async fn get_departures(
		client: &reqwest::Client,
		stop_id: &str,
		num_departures: u32,
		time: &DateTime<Local>,
	) -> Result<DepartureBoard, reqwest::Error> {
		info!(r#"Requesting departure board for "{}""#, stop_id);
		debug!(
			r#"Departure board request for "{}": {} departures from {}"#,
			stop_id,
			num_departures,
			time.to_rfc3339()
		);
		let mut vars: HashMap<&str, serde_json::Value> = HashMap::new();
		vars.insert("departures", json!(num_departures));
		vars.insert("id", json!(stop_id));
		vars.insert("startTime", json!(time.to_rfc3339()));
		let request = RequestQuery {
			query: DEPARTUREBOARD_QUERY,
			variables: vars,
		};
		let started = Instant::now();
		let response = client
			.post(JOURNEYPLANNER_URL)
			.json(&request)
			.header("ET-Client-Name", CLIENT_NAME)
			.send()
			.await?;
		log_response("Departure board", stop_id, &response, started);
		response.json::<DepartureBoard>().await
	}

	pub async fn get_stops(
		client: &reqwest::Client,
		departure_id: &str,
	) -> Result<StopList, reqwest::Error> {
		info!(r#"Requesting stops for departure "{}""#, departure_id);
		let mut vars: HashMap<&str, serde_json::Value> = HashMap::new();
		vars.insert("id", json!(departure_id));
		let request = RequestQuery {
			query: STOPS_QUERY,
			variables: vars,
		};
		let started = Instant::now();
		let response = client
			.post(JOURNEYPLANNER_URL)
			.json(&request)
			.header("ET-Client-Name", CLIENT_NAME)
			.send()
			.await?;
		log_response("Stops", departure_id, &response, started);
		response.json::<StopList>().await
	}
}

fn log_response(label: &str, query: &str, response: &reqwest::Response, started: Instant) {
	let status = response.status();
	if status.is_success() {
		debug!(
			r#"{label} for "{}" responded {} in {:?}"#,
			query,
			status,
			started.elapsed()
		);
	} else {
		warn!(r#"{label} for "{}" failed with status {}"#, query, status);
	}
}
