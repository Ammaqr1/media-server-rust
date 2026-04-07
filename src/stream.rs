use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};

pub async fn stream_video(Path(file_name): Path<String>, headers: HeaderMap) -> impl IntoResponse {
    let file_path = format!("videos/{}", file_name);

    let mut file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => return (StatusCode::NOT_FOUND, "Video not found").into_response(),
    };

    let metadata = file.metadata().await.unwrap();
    let file_size = metadata.len();

    let range = headers
        .get(header::RANGE)
        .and_then(|value| value.to_str().ok());

    if let Some(range) = range {
        let range = range.replace("bytes=", "");
        let parts: Vec<&str> = range.split("-").collect();

        let start: u64 = parts[0].parse().unwrap_or(0);
        let end: u64 = parts
            .get(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(file_size - 1);

        let chunk_size = end - start + 1;

        file.seek(SeekFrom::Start(start)).await.unwrap();

        let mut buffer = vec![0; chunk_size as usize];
        file.read_exact(&mut buffer).await.unwrap();

        return (
            StatusCode::PARTIAL_CONTENT,
            [
                (header::CONTENT_TYPE, "video/mp4"),
                (
                    header::CONTENT_RANGE,
                    &format!("bytes {}-{}/{}", start, end, file_size),
                ),
                (header::ACCEPT_RANGES, "bytes"),
                (header::CONTENT_LENGTH, &chunk_size.to_string()),
            ],
            buffer,
        )
            .into_response();
    }

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.unwrap();

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "video/mp4")],
        buffer,
    )
        .into_response()
}
