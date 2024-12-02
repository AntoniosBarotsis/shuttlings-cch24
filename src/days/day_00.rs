use axum::{
  http::{header, StatusCode},
  response::IntoResponse,
  routing::get,
  Router,
};

pub fn get_routes() -> Router {
  Router::new()
    .route("/", get(task_1))
    .route("/-1/seek", get(task_2))
}

async fn task_1() -> &'static str {
  "Hello, bird!"
}

async fn task_2() -> impl IntoResponse {
  (
    StatusCode::FOUND,
    [(
      header::LOCATION,
      "https://www.youtube.com/watch?v=9Gc4QTqslN4",
    )],
  )
}
