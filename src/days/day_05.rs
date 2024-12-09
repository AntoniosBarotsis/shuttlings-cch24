use std::str::FromStr;

use axum::{
  http::{HeaderMap, StatusCode},
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use cargo_manifest::Manifest;
use serde::{Serialize, Serializer};
use serde_with::{serde_as, skip_serializing_none, DefaultOnError};

pub fn get_routes() -> Router {
  Router::new()
    .route("/5/manifest", post(task_1))
    .route("/5", get(task_2))
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Data {
  package: Package,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Package {
  name: String,
  keywords: Vec<String>,
  metadata: Metadata,
}

#[serde_as]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Metadata {
  #[serde_as(deserialize_as = "Vec<DefaultOnError>")]
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  orders: Vec<Option<Order>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[skip_serializing_none]
struct Order {
  item: String,
  quantity: u32,
}

// impl Serialize for Metadata {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//       S: Serializer,
//   {
//       let filtered_values: Vec<Order> = self.orders.iter()
//           .filter_map(|opt| opt.as_ref()) // Keep only Some values
//           .cloned() // Clone the values (if T is not Copy)
//           .collect();

//       // Serialize the filtered values
//       filtered_values.serialize(serializer)
//   }
// }

static ALLOWED_CONTENT_TYPES: [&str; 3] = ["application/yaml", "application/json", "application/toml"];

#[allow(clippy::unwrap_used)]
async fn task_1(headers: HeaderMap, data: String) -> Result<String, impl IntoResponse> {
  let content_type = headers.get("Content-Type").unwrap().to_str().unwrap();

  if !ALLOWED_CONTENT_TYPES.contains(&content_type) {
    return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
  }

  let data = match content_type {
    "application/yaml" => {
      let mut parsed = serde_yaml::from_str::<Data>(&data).map_err(|_e| StatusCode::NO_CONTENT.into_response())?;
      parsed.package.metadata.orders.retain(Option::is_some);
      toml::to_string(&parsed).unwrap()
    },
    "application/json" => {
      let mut parsed = serde_json::from_str::<Data>(&data).map_err(|_e| StatusCode::NO_CONTENT.into_response())?;
      parsed.package.metadata.orders.retain(Option::is_some);
      toml::to_string(&parsed).unwrap()
    },
    "application/toml" => data,
    _ => unreachable!()
  };

  dbg!(&data);
  // dbg!(toml::from_str::<cargo_manifest::Package>(&data));
  // match toml::from_str::<cargo_manifest::Package>(&data) {
  //   Ok(package) => {
  //     let err_res = Err((StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response());

  //     if let Some(keywords) = package.keywords {
  //       match keywords {
  //         cargo_manifest::MaybeInherited::Inherited {
  //           workspace: _workspace,
  //         } => todo!(),
  //         cargo_manifest::MaybeInherited::Local(keywords) => {
  //           if !keywords.contains(&"Christmas 2024".to_string()) {
  //             return err_res;
  //           }
  //         }
  //       }
  //     } else {
  //       return err_res;
  //     }
  //   },
  //   Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid manifest").into_response()),
  // }
  match Manifest::from_str(&data) {
    Ok(manifest) => {
      let err_res = Err((StatusCode::BAD_REQUEST, "Magic keyword not provided").into_response());

      if let Some(keywords) = manifest.package.and_then(|p| p.keywords) {
        match keywords {
          cargo_manifest::MaybeInherited::Inherited {
            workspace: _workspace,
          } => todo!(),
          cargo_manifest::MaybeInherited::Local(keywords) => {
            if !keywords.contains(&"Christmas 2024".to_string()) {
              return err_res;
            }
          }
        }
      } else {
        return err_res;
      }
    },
    Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid manifest").into_response()),
  }

  dbg!(&data);
  dbg!(toml::from_str::<Data>(&data));
  let parsed = toml::from_str::<Data>(&data).map_err(|_e| StatusCode::NO_CONTENT.into_response())?;

  if parsed.package.metadata.orders.iter().all(Option::is_none) {
    return Err(StatusCode::NO_CONTENT.into_response());
  }

  let res = &parsed
    .package
    .metadata
    .orders
    .into_iter()
    .flatten()
    .map(|order| format!("{}: {}", order.item, order.quantity))
    .collect::<Vec<_>>()
    .join("\n");

  Ok(res.to_string())
}

async fn task_2() -> Result<String, StatusCode> {
  todo!()
}
