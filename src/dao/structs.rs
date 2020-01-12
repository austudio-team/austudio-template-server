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

pub struct CreateVersionRecord {
  marjio_version: i64,
  minor_version: i64,
  build_number: i64,
  description: String,
  branch_name: String,
}

pub struct ResultResponse {
  status: i64,
  msg: String,
}
