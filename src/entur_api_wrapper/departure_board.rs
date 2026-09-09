use std::time::Duration;

use chrono::{Local, TimeDelta};
use tokio::time::sleep;

use crate::entur_api_wrapper::{api, error::ApiResult};

#[derive(Debug, Clone)]
pub struct Departure {
	pub line: String,
	pub destination: String,
	pub time: chrono::DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct Stop {
	pub name: String,
	pub time: chrono::DateTime<Local>,
}

impl Departure {
	pub async fn get_stops(&self) -> Vec<Stop> {
		sleep(Duration::from_millis(250)).await;
		[
			"Ratesvingen",
			"Fossegrenda",
			"Nordslettvegen",
			"Nordslettvegen Terrasse",
			"Nordslettvegen snuplass",
			"Nordslettvegen Terrasse",
			"Utleirmark",
			"Astronomvegen",
			"Dalsaunevegen",
			"Nidarvoll skole",
			"Siemens",
			"Bratsbergvegen",
			"Valøyvegen",
			"Lerkendal 1",
			"Hesthagen",
			"Studentersamfundet 1",
			"Nidarosdomen",
			"Prinsens gate P2",
			"Søndre gate",
			"Trondheim S 13",
			"Dyre Halses gate",
			"Buran 2",
			"Rønningsbakken",
			"Dalen Hageby",
			"Strindheim 2",
			"Strindheim skole",
			"Strindheim Hageby",
			"Bromstadsvingen",
			"Gartnerhallen",
			"Iskremfabrikken",
			"Trondheim fengsel",
			"Hallfred Høyems veg",
			"Angelltrøvegen",
			"Sildråpevegen",
			"Granåsen gård",
			"Ramstad",
			"Stokkan",
			"Jakobsli",
			"Fortunalia",
			"Sæterbakken",
		]
		.into_iter()
		.enumerate()
		.map(|(n, s)| Stop {
			name: String::from(s),
			time: self.time + TimeDelta::minutes(2 * (n as i64 - 10)),
		})
		.collect()
	}
}

pub async fn get_departures(from: &str) -> ApiResult<Vec<Departure>> {
	let client = reqwest::Client::new();
	let result: Vec<Departure> = api::JourneyPlanner::get_departures(&client, from)
		.await?
		.data
		.stop_place
		.estimated_calls
		.into_iter()
		.map(|call| Departure {
			destination: call.destination_display.front_text,
			line: call.service_journey.journey_pattern.line.name,
			time: call
				.expected_departure_time
				.parse()
				.unwrap_or_else(|_| Local::now()),
		})
		.collect();
	Ok(result)
}
