use actix_web::{HttpRequest};
use actix_files::NamedFile;
use crate::prelude::Result;

pub async fn static_files(req: HttpRequest) -> Result<NamedFile> {
  let path = req.match_info().query("filename");
  match path {
    "template/bundle.js" => {
      let path: std::path::PathBuf = "frontend/dist/bundle.js".parse().unwrap();
      let file = NamedFile::open(path)?;
      Ok(file.use_last_modified(true))
    },
    _ => {
      let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
      Ok(NamedFile::open(path)?.use_last_modified(true))
    }
  }
}
