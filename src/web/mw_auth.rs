use crate::{Error, Result};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn mw_require_auth(
	req: Request<Body>,
	next: Next,
) -> Result<Response> {
	println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

	let auth = req.headers()
        .get("Authorization")
        .ok_or(Error::InvalidArgument)?;

    let auth_key = auth.to_str().map_err(|_| Error::InvalidArgument)?;

    let Ok(key) = std::env::var("AUTHORIZATION_KEY")
    else {
        return Err(Error::NoAuthKey);
    };

    if auth_key != key {
        return Err(Error::InvalidAuthKey);
    }

	Ok(next.run(req).await)
}