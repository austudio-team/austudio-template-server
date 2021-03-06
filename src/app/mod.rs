pub mod version;
pub mod static_files;
pub mod index;
mod not_found;
use not_found::not_found;
pub use static_files::{*};
pub use version::{*};
pub use index::{*};
use actix_web::{web, App, HttpServer, HttpResponse, guard, HttpRequest};
use actix::{Addr, SyncArbiter};
use crate::db::{DbExecutor, new_connection};
use std::env;
use std::sync::Mutex;
use crate::error::Error;

pub struct AppState {
  pub db: Addr<DbExecutor>,
  pub last_version: Mutex<Option<BasicVersion>>,
  pub token: String,
}

#[actix_rt::main]
pub async fn start() -> std::io::Result<()> {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let token = env::var("UPLOAD_TOKEN").expect("UPLOAD_TOKEN must be set");
  let addr = SyncArbiter::start(num_cpus::get(), move || new_connection(&database_url).unwrap());

  HttpServer::new(move || {
    App::new()
      .data(AppState {
        db: addr.clone(),
        last_version: Mutex::new(None),
        token: token.clone(),
      })
      .configure(routes)
      // default
      .default_service(
        // 404 for GET request
      web::resource("")
        .route(web::get().to(not_found))
        // all requests that are not `GET`
        .route(
          web::route()
            .guard(guard::Not(guard::Get()))
            .to(HttpResponse::MethodNotAllowed),
        ),
      )
    })
    .bind("0.0.0.0:6060")?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
    app
      .service(web::scope("template/api")
        .service(web::resource("versions")
          .route(web::get().to(version::list))
          .route(web::post().to(version::create))
        )
      )
      .service(web::resource("/").route(web::get().to(index)))
      .service(web::resource("/{filename:.*}").route(web::get().to(static_files)));
}
