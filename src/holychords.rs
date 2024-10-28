use crate::songs;
use axum::response::Json;
use axum::routing::{get, Router};

pub fn routes() -> Router {
    Router::new().route("/holychords", get(holychords_handler))
}

async fn holychords_handler() -> Json<Vec<songs::Song>> {
    let songs = songs::_get_mock_songs();
    Json(songs)
}
