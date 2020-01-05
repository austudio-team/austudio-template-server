use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod dao;
use dao::Dao;

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello, World")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let db = Dao::init().unwrap();
  HttpServer::new(|| {
    App::new().route("/", web::get().to(index))
  })
  .bind("0.0.0.0:6060")?
  .run()
  .await
}
