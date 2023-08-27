use crate::graphql::users::types::user::User;
use actix_web::{
    error,
    http::header::{ContentType, HeaderMap, AUTHORIZATION},
    Result,
};
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

pub fn get_user_from_headers(headers: &HeaderMap) -> Result<Option<User>> {
    if let Some(authorization) = headers.get(AUTHORIZATION) {
        let auth_str = match authorization.to_str() {
            Ok(val) => Ok(val),
            Err(err) => Err(error::ErrorImATeapot(err)),
        }?;

        let auth = match auth_str.split_once(" ") {
            Some(val) => Ok(val),
            None => Err(AuthErr::new("Invalid Authorization token")),
        }?;

        if auth.0 == "Bearer" {
            Ok(Some(User::new(01, String::from(auth.1))))
        } else {
            Err(AuthErr::new("Invalid Authorization token"))
        }
    } else {
        Ok(None)
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Auth Error: {}", name)]
pub struct AuthErr {
    name: &'static str,
}

impl ResponseError for AuthErr {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self.to_string())
    }
}

impl AuthErr {
    pub fn new(name: &'static str) -> actix_web::Error {
        Self { name }.into()
    }
}
