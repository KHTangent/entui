#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestQuery<'a> {
	pub query: &'a str,
	pub variables: HashMap<&'a str, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartureBoard {
	pub data: Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	pub stop_place: StopPlace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopPlace {
	pub id: String,
	pub name: String,
	pub estimated_calls: Vec<EstimatedCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedCall {
	pub realtime: bool,
	pub aimed_departure_time: String,
	pub expected_departure_time: String,
	pub for_boarding: bool,
	pub destination_display: DestinationDisplay,
	pub quay: Quay,
	pub service_journey: ServiceJourney,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinationDisplay {
	pub front_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quay {
	pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceJourney {
	pub journey_pattern: JourneyPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyPattern {
	pub line: Line,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
	pub id: String,
	pub public_code: String,
	pub name: String,
	pub transport_mode: String,
}
