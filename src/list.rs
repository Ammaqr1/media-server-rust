use axum::response::Json;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct VideoList {
    videos: Vec<String>,
}

pub async fn list_videos() -> Json<VideoList> {

    let mut videos = vec![];

    let entries = fs::read_dir("videos").unwrap();

    // println!("The entries are {entries}");

    for entry in entries {
        let entry = entry.unwrap();
        let name = entry.file_name();

        videos.push(name.to_string_lossy().to_string());
    }

    Json(VideoList { videos })
}