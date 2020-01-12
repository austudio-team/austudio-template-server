mod structs;
mod dao;
use structs::{CreateVersionRecord, VersionRecord, ResultResponse};
use rusqlite::Error;
use dao::Dao;
use actix::prelude::*;

impl Message for CreateVersionRecord {
  type Result = Result<ResultResponse, Error>;
}

impl Handler<CreateVersionRecord> for Dao {
  type Result = Result<ResultResponse, Error>;

  fn Handler(&mut self, msg: CreateVersionRecord, _: &mut Self::Context) -> Self::Result {
    let {
      marjio_version,
      minor_version,
      build_number,
      description,
      branch_name,
    } = CreateVersionRecord;
    let mut stmt = self.0.prepare(
      "INSERT INTO versions (
        marjio_version,
        minor_version,
        build_number,
        description,
        branch_name
      ) VALUES (?1, ?2, ?3, ?4, ?5)"
    ).expect("Prepare Error");
    match stmt.insert(&[&marjio_version.to_string(), &minor_version.to_string(), &build_number.to_string(), &description.to_string(), &branch_name.to_string()]) {
      Ok() -> ResultResponse { status: 0, msg: String::from('') }
      Err(e) -> ResultResponse { status: 0, msg: String::from(format!("{:?}", e))}
    }
  }
}
