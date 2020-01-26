use chrono::NaiveDateTime;
use crate::schema::{versions};

#[derive(Debug, Queryable, Serialize)]
pub struct Version {
  pub id: i32,
  pub major_version: i32,
  pub minor_version: i32,
  pub build_number: i32,
  pub description: String,
  pub branch_name: String,
  pub created_time: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "versions"]
pub struct NewVersion {
  pub major_version: i32,
  pub minor_version: i32,
  pub build_number: i32,
  pub description: String,
  pub branch_name: String,
}
