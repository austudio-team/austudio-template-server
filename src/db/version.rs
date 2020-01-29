use actix::prelude::*;
use diesel::prelude::*;
use crate::prelude::*;
use crate::models::{Version, CreateVersion};
use crate::app::{GetVersions, VersionResponse, CreateVersionResponse};
use crate::db::DbExecutor;
use crate::schema::{versions};

impl Message for GetVersions {
  type Result = Result<VersionResponse>;
}

impl Handler<GetVersions> for DbExecutor {
  type Result = Result<VersionResponse>;

  fn handle(&mut self, msg: GetVersions, _context: &mut Self::Context) -> Self::Result {
    let conn = &self.0;
    let limit = msg.limit.unwrap_or(20) as i64;
    let offset = msg.offset.unwrap_or( 0) as i64;

    let mut query = versions::dsl::versions.into_boxed();

    if let Some(q) = msg.q {
      let num: i32 = *(&q.parse().unwrap_or(-1));
      query = query.filter(versions::build_number.eq(num))
                   .filter(versions::branch_name.like(q.clone()))
                   .filter(versions::description.like(q.clone()));
    }

    let result = query
                  .order(versions::id.desc())
                  .limit(limit)
                  .offset(offset)
                  .load::<Version>(conn)?;
    Ok(VersionResponse {
      versions_count: result.len(),
      versions: result,
    })
  }
}

impl Message for CreateVersion {
  type Result = Result<CreateVersionResponse>;
}

impl Handler<CreateVersion> for DbExecutor {
  type Result = Result<CreateVersionResponse>;

  fn handle(&mut self, msg: CreateVersion, _context: &mut Self::Context) -> Self::Result {
    let conn = &self.0;
    diesel::insert_into(versions::table)
      .values(msg)
      .execute(conn)?;
    Ok(CreateVersionResponse {
      success: true
    })
  }
}
