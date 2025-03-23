use axum::{body::Body, http::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn mw_logging_request(req: Request<Body>, next: Next) -> Response {
    info!("Received a {} request to {}", req.method(), req.uri());
    next.run(req).await
}
