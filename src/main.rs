use axum::{routing::get_service, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod web;
mod error;
mod model;

#[tokio::main]
async fn main() -> Result<()>{
    let routes_all = new_app();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("->> {:12} on {:?}\n", "LISTENING", listener.local_addr());
    
	axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}

pub(crate) fn new_app() -> Router {
    Router::new()
    .fallback_service(routes_static())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
