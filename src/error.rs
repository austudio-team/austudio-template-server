use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use actix::MailboxError;
use diesel::{
  result::{DatabaseErrorKind, Error as DieselError},
};
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::convert::From;
use validator::ValidationErrors;

const empty_template: &'static str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="X-UA-Compatible" content="ie=edge">
  <title>austudio-template-server</title>
</head>
<body>
  <h1>404 Not Found</h1>
  <p>Sorry, no template file was found, please create a version or choose a correct version.</p>
  <script src="/template/bundle.js" defer></script>
</body>
</html>
"#;

#[derive(Fail, Debug)]
pub enum Error {
  // 401
  #[fail(display = "Unauthorized: {}", _0)]
  Unauthorized(JsonValue),

  // 403
  #[fail(display = "Forbidden: {}", _0)]
  Forbidden(JsonValue),

  // 404
  #[fail(display = "Not Found: {}", _0)]
  NotFound(JsonValue),

  // 422
  #[fail(display = "Unprocessable Entity: {}", _0)]
  UnprocessableEntity(JsonValue),

  // 500
  #[fail(display = "Internal Server Error")]
  InternalServerError,

  #[fail(display = "Template Not Found")]
  TemplateNotFound,
}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/
impl ResponseError for Error {
  fn error_response(&self) -> HttpResponse {
    match *self {
      Error::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
      Error::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
      Error::NotFound(ref message) => HttpResponse::NotFound().json(message),
      Error::UnprocessableEntity(ref message) => {
        HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message)
      }
      Error::InternalServerError => {
        HttpResponse::InternalServerError().json("Internal Server Error")
      }
      Error::TemplateNotFound => {
        HttpResponse::NotFound().body(empty_template)
      }
    }
  }
}

impl From<MailboxError> for Error {
  fn from(_error: MailboxError) -> Self {
    Error::InternalServerError
  }
}

impl From<DieselError> for Error {
  fn from(error: DieselError) -> Self {
    match error {
      DieselError::DatabaseError(kind, info) => {
        if let DatabaseErrorKind::UniqueViolation = kind {
          let message = info.details().unwrap_or_else(|| info.message()).to_string();
          return Error::UnprocessableEntity(json!({ "error": message }));
        }
        Error::InternalServerError
      }
      DieselError::NotFound => {
        Error::NotFound(json!({ "error": "requested record was not found" }))
      }
      _ => Error::InternalServerError,
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(_error: std::io::Error) -> Self {
    Error::NotFound(json!({ "error": "file not exist" }))
  }
}

impl From<ValidationErrors> for Error {
  fn from(errors: ValidationErrors) -> Self {
    let mut err_map = JsonMap::new();

    // transforms errors into objects that err_map can take
    for (field, errors) in errors.field_errors().iter() {
      let errors: Vec<JsonValue> = errors
        .iter()
        .map(|error| {
          // dbg!(error) // <- Uncomment this if you want to see what error looks like
          json!(error.message)
        })
        .collect();
      err_map.insert(field.to_string(), json!(errors));
    }

    Error::UnprocessableEntity(json!({
            "errors": err_map,
        }))
  }
}
