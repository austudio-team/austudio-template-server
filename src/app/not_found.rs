use crate::prelude::Result;
use crate::error::Error;
use actix_web::{HttpResponse};

pub async fn not_found() -> Result<HttpResponse> {
  Err(Error::TemplateNotFound)
}
