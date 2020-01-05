use rusqlite::{params, Connection, Result, NO_PARAMS};

#[derive(Debug)]
pub struct VersionRecord {
  id: i64,
  marjio_version: i64,
  minor_version: i64,
  build_number: i64,
  description: String,
  branch_name: String,
  created_time: String,
}

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

  pub fn insert_new_version(&self, marjio_version: i64, minor_version: i64, build_number: i64, description: String, branch_name: String) -> Result<i64> {
    let mut stmt = self.conn.prepare(
      "INSERT INTO versions (
        marjio_version,
        minor_version,
        build_number,
        description,
        branch_name
      ) VALUES (?1, ?2, ?3, ?4, ?5)"
    ).expect("Prepare Error");
    stmt.insert(&[&marjio_version.to_string(), &minor_version.to_string(), &build_number.to_string(), &description.to_string(), &branch_name.to_string()])
  }

  pub fn get_last_version(&self) -> Result<VersionRecord> {
    self.conn.query_row(
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
