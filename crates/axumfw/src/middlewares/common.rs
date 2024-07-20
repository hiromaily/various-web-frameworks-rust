use axum::{http::Request, Router};
use http::Method;
use tower_http::compression::predicate::{NotForContentType, Predicate, SizeAbove};
use tower_http::{
    compression::CompressionLayer, cors::Any, cors::CorsLayer, trace::DefaultOnResponse,
    trace::TraceLayer,
};
use tracing::{info_span, Level};

pub fn apply_middleware(router: Router) -> Router {
    Router::new()
        .merge(router)
        .layer({
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any)
        })
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        path = ?request.uri(),
                    )
                })
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            CompressionLayer::new().compress_when(
                // Don't compress below 512 bytes
                SizeAbove::new(512)
                    // Don't compress gRPC
                    .and(NotForContentType::GRPC)
                    // Don't compress images
                    .and(NotForContentType::IMAGES),
            ),
        )
}
