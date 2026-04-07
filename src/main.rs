use axum::{
    Router,
    routing::{get, post,get_service},
};
use tower_http::services::{ServeDir,ServeFile};
mod stream;
mod upload;
mod list;

#[tokio::main]

async fn main() {
    let app = Router::<()>::new()
        .route("/", get_service(ServeFile::new("static/index.html")))
        .route("/upload", post(upload::upload_video))
        .route("/videos",get(list::list_videos))
        .route("/stream/:filename", get(stream::stream_video))
        .nest_service("/static",ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on the port 3000");

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Rust Media Server Running"
}
