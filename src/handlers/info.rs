use serde_json::json;
use worker::*;

pub fn handle_info(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let info = json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "repository": env!("CARGO_PKG_REPOSITORY"),
        "license": env!("CARGO_PKG_LICENSE"),
        "authors": env!("CARGO_PKG_AUTHORS"),
        "date": "6 Jul 2025",
    });
    Response::from_json(&info)
}
