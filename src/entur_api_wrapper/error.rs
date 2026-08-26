#![allow(dead_code)]
use std::fmt;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(fmt::Debug, Clone)]
pub struct ApiError {
	pub kind: ApiErrorKind,
	message: String,
}

#[derive(fmt::Debug, Clone)]
pub enum ApiErrorKind {
	NoResponse,
	BadRequest,
	Unauthorized,
	Forbidden,
	NotFound,
	InternalServerError,
}

impl fmt::Display for ApiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.kind {
			ApiErrorKind::NoResponse => write!(f, "No status received"),
			ApiErrorKind::BadRequest => write!(f, "Bad Request: {}", self.message),
			ApiErrorKind::Unauthorized => write!(f, "Unauthorized: {}", self.message),
			ApiErrorKind::Forbidden => write!(f, "Forbidden: {}", self.message),
			ApiErrorKind::NotFound => write!(f, "Not Found: {}", self.message),
			ApiErrorKind::InternalServerError => {
				write!(f, "Internal Server Error: {}", self.message)
			}
		}
	}
}

impl From<reqwest::Error> for ApiError {
	fn from(value: reqwest::Error) -> Self {
		use reqwest::StatusCode;
		match value.status() {
			Some(StatusCode::BAD_REQUEST) => Self::bad_request(value.to_string()),
			Some(StatusCode::UNAUTHORIZED) => Self::unauthorized(value.to_string()),
			Some(StatusCode::FORBIDDEN) => Self::forbidden(value.to_string()),
			Some(StatusCode::NOT_FOUND) => Self::not_found(value.to_string()),
			Some(status) if status.is_server_error() => Self::internal(value.to_string()),
			Some(_) => Self::bad_request(value.to_string()),
			None => Self {
				kind: ApiErrorKind::NoResponse,
				message: String::new(),
			},
		}
	}
}

impl ApiError {
	pub fn bad_request(e: impl ToString) -> Self {
		Self {
			kind: ApiErrorKind::BadRequest,
			message: e.to_string(),
		}
	}

	pub fn unauthorized(e: impl ToString) -> Self {
		Self {
			kind: ApiErrorKind::Unauthorized,
			message: e.to_string(),
		}
	}

	pub fn forbidden(e: impl ToString) -> Self {
		Self {
			kind: ApiErrorKind::Forbidden,
			message: e.to_string(),
		}
	}

	pub fn not_found(e: impl ToString) -> Self {
		Self {
			kind: ApiErrorKind::NotFound,
			message: e.to_string(),
		}
	}

	pub fn internal(e: impl ToString) -> Self {
		Self {
			kind: ApiErrorKind::InternalServerError,
			message: e.to_string(),
		}
	}
}
