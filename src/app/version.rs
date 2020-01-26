use crate::models::{Version, NewVersion};
use crate::utils::auth::{Auth, authenticate};
use super::AppState;
use actix_web::{Responder, web, HttpResponse, ResponseError};
use crate::error::Error;
use actix_web::middleware::errhandlers::ErrorHandlerResponse::Response;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionResponse {
  pub versions: Vec<Version>,
  pub versions_count: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetVersions {
  pub q: Option<String>,
  pub limit: Option<usize>,  // <- if not set, is 20
  pub offset: Option<usize>, // <- if not set, is 0
}

pub async fn list(state: web::Data<AppState>, params: web::Query<GetVersions>) -> impl Responder {
  let db = state.db.clone();
  let get_version_params = params.0;
  let res = db.send(get_version_params).await;
  match res {
    Ok(res) => {
      match res {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => e.error_response(),
      }
    },
    Err(e) => {
      let error = Error::from(e);
      error.error_response()
    },
  }
}
