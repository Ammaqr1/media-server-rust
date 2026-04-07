use axum::{extract::Multipart, response::IntoResponse};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn upload_video(mut multipart: Multipart) -> impl IntoResponse {
    let mut invalid_file: bool = false;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let Some(file_name) = field.file_name() else {
            invalid_file = true;
            println!("Invalid file");
            continue;
        };

        let file_name = file_name.trim();

        if file_name.is_empty() {
            println!("Invalid file");
            invalid_file = true;
            continue;
        }

        let file_name = file_name.to_string();

        println!("The file_name is {file_name}");
        let data = field.bytes().await.unwrap();

        let file_path = format!("videos/{}", file_name);
        let mut file = File::create(file_path).await.unwrap();
        file.write_all(&data).await.unwrap();
    }
    if invalid_file {
        "Invalid file please check the file before sending"
    } else {
        "Video file upload successfully"
    }
}
