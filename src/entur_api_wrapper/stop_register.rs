use crate::entur_api_wrapper::api;

#[derive(Debug, Clone)]
pub struct StopSearchResult {
	pub id: String,
	pub label: String,
}

impl StopSearchResult {
	pub async fn search(query: &str) -> Vec<Self> {
		let client = reqwest::Client::new();
		let result = api::Geocoder::autocomplete(&client, query).await.unwrap();
		result
			.features
			.into_iter()
			.map(|feature| StopSearchResult {
				id: feature.properties.id,
				label: format!(
					"{} ({})",
					feature.properties.names.names_default, feature.properties.address.locality
				)
				.to_string(),
			})
			.collect()
	}
}
