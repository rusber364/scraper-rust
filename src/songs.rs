use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::{
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: Option<String>,
    pub author: Option<String>,
    pub lyrics: Option<String>,
}

pub type SongsMap = HashMap<String, Song>;
pub type SharedSongs = Arc<Mutex<SongsMap>>;

pub fn _get_mock_songs() -> Vec<Song> {
    vec![
        Song {
            id: "1".to_string(),
            title: "Псалом 1".to_string(),
            artist: Some("Артист 1".to_string()),
            author: Some("Автор 1".to_string()),
            lyrics: Some("Текст псалма 1".to_string()),
        },
        Song {
            id: "2".to_string(),
            title: "Псалом 2".to_string(),
            artist: Some("Артист 2".to_string()),
            author: Some("Автор 2".to_string()),
            lyrics: Some("Текст псалма 2".to_string()),
        },
    ]
}

pub fn _get_mock_song() -> Song {
    Song {
        id: "1".to_string(),
        title: "Псалом 1".to_string(),
        artist: Some("Артист 1".to_string()),
        author: Some("Автор 1".to_string()),
        lyrics: Some("Текст псалма 1".to_string()),
    }
}

pub fn load_songs_into_memory(file_path: &str) -> SharedSongs {
    let full_path = env::current_dir()
        .expect("Failed to get current directory")
        .join(file_path);
    let mut file = File::open(&full_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let songs = serde_json::from_str(&contents).expect("Failed to parse songs from JSON");

    Arc::new(Mutex::new(songs))
}
