use axum::response::Html;

pub async fn health() -> Html<&'static str> {
    Html("OK")
}

// TODO: remove after implementation done
pub async fn dummy() -> Html<&'static str> {
    Html("dymmy")
}
