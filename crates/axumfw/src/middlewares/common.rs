use axum::Router;
use http::Method;
use tower_http::{cors::Any, cors::CorsLayer, trace::TraceLayer};

pub fn apply_middleware(router: Router) -> Router {
    Router::new()
        .merge(router)
        .layer({
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any)
        })
        .layer(TraceLayer::new_for_http())
}
