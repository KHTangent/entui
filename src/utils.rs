use chrono::{DateTime, TimeZone, Timelike};

pub fn format_relative_time<Tz: TimeZone>(base: &DateTime<Tz>, other: &DateTime<Tz>) -> String {
	let timediff = other.to_utc() - base.to_utc();
	match timediff.num_minutes() {
		0 => "now".to_string(),
		n @ 1..=10 => format!("{n} minutes"),
		_ => format!("{:02}:{:02}", other.hour(), other.minute()),
	}
}
