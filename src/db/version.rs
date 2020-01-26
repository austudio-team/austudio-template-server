use actix::prelude::*;
use diesel::prelude::*;
use crate::prelude::*;
use crate::models::{Version, NewVersion};
use crate::app::{GetVersions, VersionResponse};
use crate::db::DbExecutor;
use crate::schema::{versions};


impl Message for GetVersions {
  type Result = Result<VersionResponse>;
}

impl Handler<GetVersions> for DbExecutor {
  type Result = Result<VersionResponse>;

  fn handle(&mut self, msg: GetVersions, context: &mut Self::Context) -> Self::Result {
    let conn = &self.0;
    let result = versions::dsl::versions.load::<Version>(conn)?;
    Ok(VersionResponse {
      versions_count: result.len(),
      versions: result,
    })
  }
}
