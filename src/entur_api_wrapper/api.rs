use std::collections::HashMap;

use serde_json::json;

use crate::entur_api_wrapper::raw_types::{
	geocoding::AutocompleteResponse,
	journey_planner::{DepartureBoard, RequestQuery},
};

const GEOCODER_URL: &'static str = "https://api.entur.io/geocoder/v3/autocomplete";
const JOURNEYPLANNER_URL: &'static str = "https://api.entur.io/journey-planner/v3/graphql";
const CLIENT_NAME: &'static str = "KHTangent-Entui";

const DEPARTUREBOARD_QUERY: &'static str = r#"
query departureBoard($id: String!, $departures: Int!) {
	stopPlace(id: $id) {
		id
		name
		estimatedCalls(timeRange: 72100, numberOfDepartures: $departures) {
			realtime
				aimedDepartureTime
				expectedDepartureTime
				forBoarding
				destinationDisplay {
					frontText
				}
			quay {
				id
			}
			serviceJourney {
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

pub struct Geocoder;

impl Geocoder {
	pub async fn autocomplete(
		client: &reqwest::Client,
		query: &str,
	) -> Result<AutocompleteResponse, reqwest::Error> {
		client
			.get(GEOCODER_URL)
			.query(&[("layers", "stopPlace"), ("q", query)])
			.header("ET-Client-Name", CLIENT_NAME)
			.send()
			.await?
			.json::<AutocompleteResponse>()
			.await
	}
}

pub struct JourneyPlanner;

impl JourneyPlanner {
	pub async fn get_departures(
		client: &reqwest::Client,
		stop_id: &str,
	) -> Result<DepartureBoard, reqwest::Error> {
		let mut vars: HashMap<&str, serde_json::Value> = HashMap::new();
		vars.insert("departures", json!(10));
		vars.insert("id", json!(stop_id));
		let request = RequestQuery {
			query: DEPARTUREBOARD_QUERY,
			variables: vars,
		};
		client
			.get(JOURNEYPLANNER_URL)
			.json(&request)
			.header("ET-Client-Name", CLIENT_NAME)
			.send()
			.await?
			.json::<DepartureBoard>()
			.await
	}
}
