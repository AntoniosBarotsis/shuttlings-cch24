use std::net::{Ipv4Addr, Ipv6Addr};

use axum::{
  extract::Query,
  response::IntoResponse,
  routing::get,
  Router,
};

pub fn get_routes() -> Router {
  Router::new()
    .route("/2/dest", get(task_1))
    .route("/2/key", get(task_2))
    .route("/2/v6/dest", get(task_3a))
    .route("/2/v6/key", get(task_3b))
}

#[derive(serde::Deserialize)]
struct Input1 {
  from: Ipv4Addr,
  key: Ipv4Addr,
}

// from + key == dest (where "+" is overflowing addition) is applied to each of the four octets separately.
async fn task_1(input: Query<Input1>) -> String {
  let [from_a, from_b, from_c, from_d] = input.from.octets();
  let [key_a, key_b, key_c, key_d] = input.key.octets();

  format!(
    "{}.{}.{}.{}",
    from_a.overflowing_add(key_a).0,
    from_b.overflowing_add(key_b).0,
    from_c.overflowing_add(key_c).0,
    from_d.overflowing_add(key_d).0
  )
}

#[derive(serde::Deserialize)]
struct Input2 {
  from: Ipv4Addr,
  to: Ipv4Addr,
}

async fn task_2(input: Query<Input2>) -> String {
  let [from_a, from_b, from_c, from_d] = input.from.octets();
  let [to_a, to_b, to_c, to_d] = input.to.octets();

  format!(
    "{}.{}.{}.{}",
    to_a.overflowing_sub(from_a).0,
    to_b.overflowing_sub(from_b).0,
    to_c.overflowing_sub(from_c).0,
    to_d.overflowing_sub(from_d).0
  )
}

#[derive(serde::Deserialize)]
struct Input3 {
  from: Ipv6Addr,
  key: Ipv6Addr,
}

async fn task_3a(input: Query<Input3>) -> impl IntoResponse {
  let froms = input.from.segments();
  let keys = input.key.segments();
  let tmp = froms
    .iter()
    .zip(keys)
    .map(|(a, b)| (a ^ b))
    .collect::<Vec<_>>();

  let res = Ipv6Addr::new(
    tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7],
  );

  res.to_string()
}

#[derive(serde::Deserialize)]
struct Input4 {
  from: Ipv6Addr,
  to: Ipv6Addr,
}

async fn task_3b(input: Query<Input4>) -> impl IntoResponse {
  let froms = input.from.segments();
  let tos = input.to.segments();
  let tmp = froms
    .iter()
    .zip(tos)
    .map(|(a, b)| (a ^ b))
    .collect::<Vec<_>>();

  let res = Ipv6Addr::new(
    tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], tmp[5], tmp[6], tmp[7],
  );

  res.to_string()
}
