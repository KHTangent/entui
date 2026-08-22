use crate::entur_api_wrapper::raw_types::geocoding::AutocompleteResponse;

const GEOCODER_URL: &'static str = "https://api.entur.io/geocoder/v3/autocomplete";
const CLIENT_NAME: &'static str = "KHTangent-Entui";

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
