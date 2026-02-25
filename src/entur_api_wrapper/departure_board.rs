use chrono::{TimeDelta, Utc};

#[derive(Debug)]
pub struct Departure {
	pub line: String,
	pub destination: String,
	pub time: chrono::DateTime<Utc>,
}

pub fn get_departures(from: &str) -> Vec<Departure> {
	match from {
		"Siemens" => {
			let mut v: Vec<Departure> = Vec::with_capacity(10);
			for i in 0..10 {
				v.push(Departure {
					line: "10".to_string(),
					destination: "Sæterbakken via Sentrum".to_string(),
					time: Utc::now() + TimeDelta::minutes(5 * i),
				});
			}
			v
		}
		_ => vec![],
	}
}
