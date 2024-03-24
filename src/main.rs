use axum::{http::{Method, Uri}, middleware, response::IntoResponse, routing::get_service, Json, Router};
use axum::response::Response;
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::{log::log_request, model::ModelController};

pub use self::error::{Error, Result};

pub mod web;
pub mod error;
pub mod model;
pub mod log;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<()>{
    let mc = ModelController::new();

    let routes_all = new_app(mc);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("->> {:12} on {:?}\n", "LISTENING", listener.local_addr());
    
	axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}

pub(crate) fn new_app(mc: ModelController) -> Router {
    let routes_apis = web::routes_main::routes(mc.clone());

    Router::new()
    .nest("/api", routes_apis)
    .layer(middleware::map_response(main_response_mapper))
    .fallback_service(routes_static())
}

async fn main_response_mapper(
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	println!("->> {:12} - main_response_mapper", "RES_MAPPER");
	let uuid = Uuid::new_v4();

	// -- Get the eventual response error.
	let service_error = res.extensions().get::<Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	// -- If client error, build the new reponse.
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});

				println!("    ->> client_error_body: {client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});

	// Build and log the server log line.
	let client_error = client_status_error.unzip().1;
	// TODO: Need to hander if log_request fail (but should not fail request)
	let _ =
		log_request(uuid, req_method, uri, service_error, client_error).await;

	println!();
	error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
