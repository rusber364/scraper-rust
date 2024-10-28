use crate::songs;
use axum::response::Json;
use axum::routing::{get, Router};

pub fn routes() -> Router {
    Router::new().route("/fonki", get(fonki_handler))
}

async fn fonki_handler() -> Json<Vec<songs::Song>> {
    let songs = songs::_get_mock_songs();
    Json(songs)
}
