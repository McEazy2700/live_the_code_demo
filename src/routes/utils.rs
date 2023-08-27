use crate::{graphql::users::{types::user::User, auth::utils::decode_token, utils::db_manager::UserManager}, context::AppContext};
use actix_web::{
    error,
    http::header::{ContentType, AUTHORIZATION},
    Result, HttpRequest, web::Data,
};
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

pub async fn get_user_from_headers(req: &HttpRequest) -> Result<Option<User>> {
    if let Some(authorization) = req.headers().get(AUTHORIZATION) {
        let auth_str = match authorization.to_str() {
            Ok(val) => Ok(val),
            Err(err) => Err(error::ErrorImATeapot(err)),
        }?;

        let auth = match auth_str.split_once(" ") {
            Some(val) => Ok(val),
            None => Err(AppErr::new("Invalid Authorization token")),
        }?;

        if auth.0 == "Bearer" {
            let decoded = match decode_token(String::from(auth.1)) {
                Ok(val) => Ok(val),
                Err(_) => Err(AppErr::new("Invalid Authorization token")),
            }?;
            let cx = match req.app_data::<Data<AppContext>>() {
                Some(cx) => Ok(cx),
                None => Err(AppErr::new("Failed to load context")),
            }?;
            let user = match UserManager::get_user_by_id(&cx.db, decoded.claims.id).await {
                Ok(usr) => Ok(usr),
                Err(_) => Err(AppErr::new("User not found")),
            }?;
            return Ok(Some(user.into()))
        } else {
            Err(AppErr::new("Invalid Authorization token"))
        }
    } else {
        Ok(None)
    }
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Auth Error: {}", name)]
pub struct AppErr {
    name: &'static str,
}

impl ResponseError for AppErr {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(self.to_string())
    }
}

impl AppErr {
    pub fn new(name: &'static str) -> actix_web::Error {
        Self { name }.into()
    }
}
