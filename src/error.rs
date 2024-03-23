use std::fmt::{Display, Formatter};
use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};

// region:      Result type
pub type Result<T> = core::result::Result<T, Error>;

// region:      Server Error
#[derive(Debug, Deserialize, Serialize, Clone, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    InvalidArgument
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:12} - {self:?}", "INTO_RESPONSE");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // insert error into response
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
		match self {
			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
    }
}

// region:      Client Error
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    SERVICE_ERROR
}