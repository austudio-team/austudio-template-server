use rusqlite::{Connection, Result, NO_PARAMS};
use actix::{Actor, prelude::*};

pub mod structs;
use structs::*;

pub struct Dao(Connection);

impl Actor for Dao {
  type Context = SyncContext<Self>;
}

impl Dao {
  pub fn init() -> Result<Dao> {
    let conn = Connection::open("template.db")?;
    conn.execute(
      "create table if not exists versions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        marjio_version INTEGER,
        minor_version INTEGER,
        build_number INTEGER,
        description TEXT,
        branch_name TEXT,
        created_time DATETIME DEFAULT (datetime(CURRENT_TIMESTAMP, 'localtime'))
      )",
      NO_PARAMS,
    )?;
    Ok(Dao(conn))
  }

  pub fn get_last_version(&self) -> Result<VersionRecord> {
    self.0.query_row(
      "select * from versions order by id DESC limit 1",
      NO_PARAMS,
      |row| Ok(
        VersionRecord {
          id: row.get(0)?,
          marjio_version: row.get(1)?,
          minor_version: row.get(2)?,
          build_number: row.get(3)?,
          description: row.get(4)?,
          branch_name: row.get(5)?,
          created_time: row.get(6)?,
        }
      )
    )
  }
}
