use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteResponse {
	#[serde(rename = "type")]
	pub autocomplete_response_type: String,
	pub features: Vec<Feature>,
	pub metadata: Metadata,
	pub bbox: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
	#[serde(rename = "type")]
	pub feature_type: FeatureType,
	pub geometry: Geometry,
	pub properties: Place,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureType {
	Feature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
	#[serde(rename = "type")]
	pub geometry_type: GeometryType,
	pub coordinates: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeometryType {
	Point,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
	pub id: String,
	pub names: Names,
	pub layer: Layer,
	pub source: Source,
	pub address: Address,
	pub categories: Vec<String>,
	pub fare_zones: Vec<String>,
	pub transport_modes: Vec<TransportMode>,
	pub stop_place_types: Vec<StopPlaceType>,
	pub stop_place_role: StopPlaceRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
	pub locality: String,
	pub locality_id: String,
	pub county: String,
	pub county_id: String,
	pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lang {
	No,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Layer {
	Address,
	Street,
	StopPlace,
	GroupOfStopPlaces,
	Poi,
	Place,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Names {
	#[serde(rename = "default")]
	pub names_default: String,
	pub display: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Source {
	Nsr,
	Openstreetmap,
	KartverketMatrikkelenadresse,
	KartverketStedsnavn,
	CustomPoi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StopPlaceRole {
	Parent,
	Child,
	Standalone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StopPlaceType {
	OnstreetBus,
	OnstreetTram,
	Airport,
	RailStation,
	MetroStation,
	BusStation,
	CoachStation,
	TramStation,
	HarbourPort,
	FerryPort,
	FerryStop,
	LiftStation,
	VehicleRailInterchange,
	Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportMode {
	pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
	pub query: Query,
	pub result_count: i64,
	pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
	pub text: String,
	pub limit: i64,
	pub lang: String,
	pub filters: Filters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filters {
	pub layers: Vec<Layer>,
}
