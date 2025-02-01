use axum::{body::Body, http::Request, middleware::Next, response::Response};

pub async fn mw_logging_request(req: Request<Body>, next: Next) -> Response {
    println!("->> {:<12} mw_logging_request", "MIDDLEWARE");
    println!("--> received a {} request to {}", req.method(), req.uri());
    next.run(req).await
}
