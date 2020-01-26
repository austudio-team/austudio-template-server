pub mod version;
pub mod static_files;
pub use static_files::{*};
pub use version::{*};
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix::{Addr, SyncArbiter};
use crate::db::{DbExecutor, new_connection};
use std::env;

pub struct AppState {
  pub db: Addr<DbExecutor>,
}

#[actix_rt::main]
pub async fn start() -> std::io::Result<()> {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let addr = SyncArbiter::start(num_cpus::get(), move || new_connection(&database_url).unwrap());

  HttpServer::new(move || {
    App::new()
      .data(AppState { db: addr.clone() })
      .configure(routes)
  })
    .bind("0.0.0.0:6060")?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app
      .service(web::scope("template/api")
        .service(web::resource("versions")
          .route(web::get().to(version::list)))
      )
      .service(web::resource("/{filename:.*}").route(web::get().to(static_files)))
}
