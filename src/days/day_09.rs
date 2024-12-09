use std::{borrow::BorrowMut, sync::{Arc, Mutex, RwLock}};

use axum::{
  extract::State, http::{header, HeaderMap, StatusCode}, response::IntoResponse, routing::{get, post}, Router
};
use leaky_bucket::RateLimiter;

fn create_bucket() -> RateLimiter {
  RateLimiter::builder()
    .interval(std::time::Duration::from_secs(1))
    .max(5)
    .initial(5)
    .build()
}

pub fn get_routes() -> Router {
  let limiter = Arc::new(RwLock::new(create_bucket()));
  
  Router::new()
    .route("/9/milk", post(tasks))
    .route("/9/refill", post(task_4))
    .with_state(MilkState { limiter })
}

#[derive(Clone)]
struct MilkState {
  limiter: Arc<RwLock<RateLimiter>>,
}

#[derive(serde::Deserialize)]
struct Data {
  gallons: Option<f32>,
  liters: Option<f32>,
  litres: Option<f32>,
  pints: Option<f32>,
}

async fn tasks(state: State<MilkState>, headers: HeaderMap, data: String) -> Result<String, impl IntoResponse> {
  if !state.limiter.read().unwrap().try_acquire(1) {
    return Err((StatusCode::TOO_MANY_REQUESTS, "No milk available\n").into_response())
  }

  let c_type = headers.get("Content-Type")
  .and_then(|c| c.to_str().ok());

  if c_type == Some("application/json") {
    let parsed = serde_json::from_str::<Data>(&data)
      .map_err(|_e| StatusCode::BAD_REQUEST.into_response())?;

    let fields = [parsed.gallons, parsed.liters, parsed.litres, parsed.pints];

    if fields.iter().all(Option::is_none) || fields.iter().filter(|el| el.is_some()).count() > 1 {
      return Err(StatusCode::BAD_REQUEST.into_response());
    }

    let liters = parsed.gallons.map(|g| g * 3.785_412_5);
    let gallons = parsed.liters.map(|g| g / 3.785_412_5);
    let pints = parsed.litres.map(|g| g * 1.759_754);
    let litres = parsed.pints.map(|g| g / 1.759_754);

    match (liters, gallons, pints, litres) {
      (Some(liters), _, _, _) => Ok(format!("{{\"liters\":{liters}}}")),
      (_, Some(gallons), _, _) => Ok(format!("{{\"gallons\":{gallons}}}")),
      (_, _, Some(pints), _) => Ok(format!("{{\"pints\":{pints}}}")),
      (_, _, _, Some(litres)) => Ok(format!("{{\"litres\":{litres}}}")),
      _ => unreachable!()
    }
  }  else {
    Ok("Milk withdrawn\n".to_string())
  }

}

async fn task_4(state: State<MilkState>) -> StatusCode {
  *state.limiter.write().unwrap() = create_bucket();

  StatusCode::OK
}