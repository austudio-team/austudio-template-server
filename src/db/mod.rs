use actix::{Actor, prelude::*};
use diesel::{Connection, sqlite::SqliteConnection, ConnectionError};

mod version;

pub struct DbExecutor(pub SqliteConnection);

impl Actor for DbExecutor {
  type Context = SyncContext<Self>;
}

pub fn new_connection(database_url: &String) -> Result<DbExecutor, ConnectionError> {
  let db = SqliteConnection::establish(database_url.as_str())?;
  Ok(DbExecutor(db))
}
