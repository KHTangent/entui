use std::time::Duration;

use chrono::{Local, TimeDelta};
use tokio::time::sleep;

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

pub async fn get_departures(from: &str) -> Vec<Departure> {
	sleep(Duration::from_millis(250)).await;
	match from {
		"Siemens" => {
			let mut v: Vec<Departure> = Vec::with_capacity(10);
			for i in 0..10 {
				v.push(Departure {
					line: "10".to_string(),
					destination: "Sæterbakken via Sentrum".to_string(),
					time: Local::now() + TimeDelta::minutes(5 * i),
				});
			}
			v
		}
		_ => vec![],
	}
}
