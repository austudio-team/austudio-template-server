use crate::models::{Version, CreateVersion};
use crate::utils::{
  last_version::get_last_version,
  extract_file::extract_file_and_inject,
};
use actix_multipart::Multipart;
use super::AppState;
use actix_web::{Responder, web, HttpResponse, ResponseError};
use crate::error::Error;
use futures::StreamExt;
use serde_json;
use actix_web::web::Bytes;
use serde::Serialize;

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

#[derive(Debug)]
pub struct BasicVersion {
  pub major_version: i32,
  pub minor_version: i32,
  pub build_number: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVersionParams {
  pub mode: Option<String>,
  pub branch_name: Option<String>,
  pub description: Option<String>,
  pub token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateVersionResponse {
  pub success: bool,
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

pub async fn create(state: web::Data<AppState>, mut payload: Multipart) -> impl Responder {
  let db = state.db.clone();
  let mut last_version = state.last_version.lock().unwrap();
  let last_version = get_last_version(&db, last_version, false).await;
  let mut create_info: Option<CreateVersionParams> = None;
  let mut file_chunk: Option<Bytes> = None;
  while let Some(item) = payload.next().await {
    let mut field = item.unwrap();
    let content_disposition = field.content_disposition().unwrap();
    let field_name = content_disposition.get_name().unwrap_or_default();
    let chunk = field.next().await.unwrap();
    match field_name {
      "info" => {
        let data = chunk.unwrap().to_vec();
        let str = String::from_utf8(data).unwrap();
        let str = str.replace("\n", "\\n");
        let str = str.replace("\r", "\\r");
        let res = serde_json::from_str::<CreateVersionParams>(&str.as_str()).unwrap();
        create_info = Some(res);
      },
      "file" => {
        let data = chunk.unwrap();
        file_chunk = Some(data);
      }
      _ => (),
    }
  }
  if create_info.is_none() || file_chunk.is_none() {
    return Error::UnprocessableEntity(json!({ "error": "invalided request" })).error_response();
  }
  let mut major_version = last_version.as_ref().map_or(1, |v| v.major_version);
  let mut minor_version = last_version.as_ref().map_or(0, |v| v.minor_version);
  let mut build_number = last_version.as_ref().map_or(0, |v| v.build_number);

  let create_info = create_info.unwrap();
  let token = create_info.token.unwrap_or(String::from("-1"));

  if !token.eq(&state.token) {
    return Error::Forbidden(json!({ "error": "token not correct" })).error_response();
  }
  if last_version.is_some() {
    let string = create_info.mode.unwrap_or(String::from("build"));
    let mode = string.as_str();
    match mode {
      "major" => {
        minor_version = 0;
        build_number = 0;
        major_version += 1;
      },
      "minor" => {
        build_number = 0;
        minor_version += 1;
      },
      _ => { build_number += 1 },
    }
  }
  let description = create_info.description.unwrap_or(String::from("No description"));
  let branch_name = create_info.branch_name.unwrap_or(String::from("no-branch"));
  let extract_res = extract_file_and_inject(
    file_chunk.unwrap(),
    format!("versions/{}/{}/{}", major_version, minor_version, build_number).as_str()
  );
  if extract_res.is_err() {
    return Error::UnprocessableEntity(json!({ "error": "file can't extract" })).error_response();
  }
  let res = db.send(CreateVersion {
    branch_name,
    description,
    build_number,
    minor_version,
    major_version,
  }).await;
  match res {
    Ok(res) => {
      get_last_version(&db, last_version, true).await;
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
