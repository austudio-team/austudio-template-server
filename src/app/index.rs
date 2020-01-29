use actix_web::{HttpRequest, web, HttpMessage, HttpResponse, Responder};
use actix_files::NamedFile;
use crate::prelude::Result;
use crate::app::{AppState, BasicVersion};
use crate::utils::last_version::{get_last_version, parse_cookie_version};
use std::path::PathBuf;
use crate::error::Error;

pub async fn index(state: web::Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
  let db = state.db.clone();
  let cookie = req.cookie("version");;
  let target_version = parse_cookie_version(cookie, &db, state.last_version.lock().unwrap()).await;
  println!("{:?}", target_version);
  match target_version {
    Some(v) => {
      let path = format!(
        "versions/{}/{}/{}/index.html",
        v.major_version,
        v.minor_version,
        v.build_number,
      );
      let path: PathBuf = path.parse().unwrap();
      match NamedFile::open(path) {
        Ok(v) => Ok(v.use_last_modified(true)),
        Err(_) => Err(Error::TemplateNotFound)
      }
    },
    None => {
      Err(Error::TemplateNotFound)
    }
  }
}
