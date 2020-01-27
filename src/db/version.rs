use actix::prelude::*;
use diesel::prelude::*;
use crate::prelude::*;
use crate::models::{Version, NewVersion};
use crate::app::{GetVersions, VersionResponse, GetLastVersion};
use crate::db::DbExecutor;
use crate::schema::{versions};

impl Message for GetVersions {
  type Result = Result<VersionResponse>;
}

impl Handler<GetVersions> for DbExecutor {
  type Result = Result<VersionResponse>;

  fn handle(&mut self, msg: GetVersions, context: &mut Self::Context) -> Self::Result {
    let conn = &self.0;
    let limit = msg.limit.unwrap_or(20) as i64;
    let offset = msg.offset.unwrap_or( 0) as i64;

    let mut query = versions::dsl::versions;

    query
      .order(versions::id.desc())
      .limit(limit)
      .offset(offset);

    if msg.q.is_some() {
      let q = msg.q.unwrap();
      query
        .filter(versions::major_version.like(q))
        .filter(versions::build_number.like(q))
        .filter(versions::branch_name.like(q))
        .filter(versions::description.like(q))
    }

    let result = query.load::<Version>(conn)?;
    Ok(VersionResponse {
      versions_count: result.len(),
      versions: result,
    })
  }
}
