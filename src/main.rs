use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix::{Addr, System, SyncArbiter};
mod dao;
use dao::Dao;

struct AppState {
  db: Addr<Dao>,
}

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello, World")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let sys = System::new("austido-template-server");

  let addr = SyncArbiter::start(3, || Dao::init().unwrap());

  HttpServer::new(move || {
    App::new()
      .app_data(AppState { db: addr.clone() })
      .route("/", web::get().to(index))
  })
  .bind("0.0.0.0:6060")?
  .run()
  .await
}
