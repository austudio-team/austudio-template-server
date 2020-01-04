use rusqlite::{params, Connection, Result, NO_PARAMS};

pub struct Dao {
  conn: Connection,
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
    Ok(Dao {
      conn
    })
  }
}
