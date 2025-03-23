use axum::{middleware, routing::post, Router};

use crate::{
    api::handlers::auth_handlers::{signin, signup},
    api::middleware::auth_middleware::mw_require_auth,
    api::state::AppState,
};

pub fn routes() -> Router<AppState> {
    let accessible_router =
        Router::new().route("/signup", post(signup)).route("/signin", post(signin));

    let restricted_router = Router::new().layer(middleware::from_fn(mw_require_auth));

    let routes = accessible_router.merge(restricted_router);
    routes
}
