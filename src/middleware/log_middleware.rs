use axum::{body::Body, http::Request, middleware::Next, response::Response};
use tracing::debug;

pub async fn mw_logging_request(req: Request<Body>, next: Next) -> Response {
    debug!("Received a {} request to {}", req.method(), req.uri());
    next.run(req).await
}
