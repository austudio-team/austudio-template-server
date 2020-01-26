use super::AppState;
use actix_web::{Responder, web, HttpResponse, HttpRequest};
use actix_files as fs;

pub async fn list(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
  let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();

}
