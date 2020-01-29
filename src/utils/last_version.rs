use crate::app::{GetVersions, BasicVersion};
use std::sync::MutexGuard;
use actix::Addr;
use crate::db::DbExecutor;
use actix_web::cookie::Cookie;

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

pub async fn parse_cookie_version<'a>(cookie: Option<Cookie<'a>>, db: &Addr<DbExecutor>, last_version: MutexGuard<'a, Option<BasicVersion>>) -> Option<BasicVersion> {
  match cookie {
    Some(cookie) => {
      let value = cookie.value();
      let version_value: Vec<&str> = value.split(".").collect();
      let major = version_value.get(0).map_or(0, |v| v.parse::<i32>().unwrap_or(0));
      let minor = version_value.get(1).map_or(0, |v| v.parse::<i32>().unwrap_or(0));
      let build = version_value.get(2).map_or(0, |v| v.parse::<i32>().unwrap_or(0));
      Some(BasicVersion {
        major_version: major,
        minor_version: minor,
        build_number: build,
      })
    },
    None => {
      let db = db.clone();
      let last_version = get_last_version(&db, last_version, false).await;
      match *last_version {
        Some(ref v) => Some(BasicVersion {
          major_version: v.major_version.clone(),
          minor_version: v.minor_version.clone(),
          build_number: v.build_number.clone(),
        }),
        None => None,
      }
    }
  }
}