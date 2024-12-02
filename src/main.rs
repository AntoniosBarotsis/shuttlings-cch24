use axum::{
  http::{
    header,
    StatusCode,
  },
  response::IntoResponse,
  routing::get,
  Router,
};

async fn hello_world() -> &'static str {
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

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> shuttle_axum::ShuttleAxum {
  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/seek", get(task_2));

  Ok(router.into())
}
