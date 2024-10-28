use crate::songs;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, Router},
};
use regex::Regex;
use serde_json::json;

#[derive(Clone)]
struct AppState {
    songs: songs::SharedSongs,
}

pub fn routes() -> Router {
    let songs = songs::load_songs_into_memory("database/songs.json");

    Router::new().nest(
        "/psalms",
        Router::new()
            .route("/:search", get(get_psalms_by_searching))
            .route("/song/:id", get(get_psalm_by_id))
            .with_state(AppState { songs }),
    )
}

async fn get_psalms_by_searching(State(state): State<AppState>, Path(search): Path<String>) -> Json<Vec<songs::Song>> {
    tracing::info!("GET -> psalms/{}", search);
    let songs = state.songs.lock().unwrap();

    let songs = {
        let search_title = Regex::new(&format!("(?i){}", regex::escape(&search))).unwrap();
        let mut filtered_ids = Vec::new();
        let mut list = Vec::new();

        for (id, song) in songs.iter() {
            if search_title.is_match(&song.title) {
                filtered_ids.push(id.clone());
                list.push(song.clone());
            }
        }

        list
    };

    Json(songs)
}

async fn get_psalm_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<songs::Song>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("GET -> psalms/song/{}", id);
    let songs = state.songs.lock().unwrap();

    if let Some(song) = songs.get(&id) {
        Ok(Json(song.clone()))
    } else {
        let error_response = Json(json!({"error": "Psalm not found"}));
        Err((StatusCode::NOT_FOUND, error_response))
    }
}
