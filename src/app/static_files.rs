use actix_web::{HttpRequest, web, HttpMessage, HttpResponse, Responder};
use actix_files::NamedFile;
use crate::prelude::Result;
use crate::app::{AppState, BasicVersion};
use crate::utils::last_version::{get_last_version, parse_cookie_version};
use std::path::PathBuf;
use crate::error::Error;

pub async fn static_files(state: web::Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
  let db = state.db.clone();
  let cookie = req.cookie("version");
  let path = req.match_info().query("filename");
  match path {
    "template/bundle.js" => {
      let path: std::path::PathBuf = "frontend/dist/bundle.js".parse().unwrap();
      let file = NamedFile::open(path)?;
      Ok(file.use_last_modified(true))
    },
    _ => {
      let target_version = parse_cookie_version(cookie, &db, state.last_version.lock().unwrap()).await;
      match target_version {
        Some(v) => {
          let path = format!(
            "versions/{}/{}/{}/{}",
            v.major_version,
            v.minor_version,
            v.build_number,
            &path,
          );
          let path: PathBuf = path.parse().unwrap();
          Ok(NamedFile::open(path)?.use_last_modified(true))
        },
        None => {
          Err(Error::TemplateNotFound)
        }
      }
    }
  }
}
