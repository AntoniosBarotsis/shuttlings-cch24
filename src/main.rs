pub mod days;

use axum::Router;

#[shuttle_runtime::main]
#[allow(clippy::unused_async)]
async fn main() -> shuttle_axum::ShuttleAxum {
  let router = Router::new()
    .merge(days::day_00::get_routes())
    .merge(days::day_02::get_routes())
    .merge(days::day_05::get_routes());

  Ok(router.into())
}
