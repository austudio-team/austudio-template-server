use std::fs;
mod dao;
use dao::Dao;

fn main() {
  let db = Dao::init().unwrap();
}
