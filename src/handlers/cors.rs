use worker::*;

// Shared CORS constants
const CORS_ORIGIN: &str = "*";
const CORS_METHODS: &str = "GET, POST, OPTIONS";
const CORS_HEADERS: &str = "Content-Type, Authorization, X-API-Key";
const CORS_MAX_AGE: &str = "86400";

pub fn handle_cors_preflight() -> Result<Response> {
    let mut response = Response::empty()?;
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", CORS_ORIGIN)?;
    headers.set("Access-Control-Allow-Methods", CORS_METHODS)?;
    headers.set("Access-Control-Allow-Headers", CORS_HEADERS)?;
    headers.set("Access-Control-Max-Age", CORS_MAX_AGE)?;
    Ok(response)
}
