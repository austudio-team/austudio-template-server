use crate::app::{GetVersions, BasicVersion};
use std::sync::MutexGuard;
use actix::Addr;
use crate::db::DbExecutor;

pub async fn get_last_version<'a>(db: &Addr<DbExecutor>, mut last_version: MutexGuard<'a, Option<BasicVersion>>, force_update: bool) -> MutexGuard<'a, Option<BasicVersion>> {
  if force_update || last_version.is_none() {
    let query = db.send(GetVersions {
      limit: Some(1),
      q: None,
      offset: Some(0),
    }).await;
    let res = query.unwrap().unwrap();
    match res.versions_count {
      0 => *last_version = None,
      _ => {
        let last = res.versions.get(0).unwrap();
        let basic_version = Some(BasicVersion {
          major_version: last.major_version.clone(),
          minor_version: last.minor_version.clone(),
          build_number: last.build_number.clone(),
        });
        *last_version = basic_version;
      }
    }
  }
  last_version
}
