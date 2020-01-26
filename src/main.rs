#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
extern crate chrono;

use dotenv;
mod db;
mod schema;
mod prelude;
mod error;
mod models;
mod app;
mod utils;

fn main() {
  dotenv::dotenv().ok();
  app::start();
}
