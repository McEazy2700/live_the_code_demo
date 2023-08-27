use anyhow::Result;
use async_graphql::*;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use std::env::var;
use crate::graphql::users::types::user::User;
use super::types::{GoogleTokenInfo, JWTClaims};

pub fn ensure_auth(cx: &Context<'_>) -> Result<User, Error> {
    let user = cx.data::<Option<User>>()?;
    match user {
        Some(user) => Ok(user.clone()),
        None => Err(Error::new(
            "Validation Error: You are not allowed to perfom this action",
        )),
    }
}

pub async fn verify_google_token(id_token: String) -> Result<GoogleTokenInfo> {
    let token_url = format!(
        "https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}",
        id_token
    );
    let token_info: GoogleTokenInfo = reqwest::get(token_url).await?.json().await?;
    Ok(token_info)
}

pub fn generate_token(user: &User) -> Result<String, Error> {
    let issuer = var("JWT_ISSUER").expect("JWT_ISSUER");
    let secret = var("SECRET").expect("SECRET");
    let exp = Utc::now() + Duration::minutes(10);
    let header = Header::new(Algorithm::HS256);
    
    let claims = JWTClaims {
        id: user.id,
        exp: exp.timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
        iss: issuer,
    };
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

pub fn decode_token(token: String) -> Result<TokenData<JWTClaims>, Error> {
    let secret = var("SECRET").expect("SECRET");
    let validation = Validation::new(Algorithm::HS256);
    let token = decode::<JWTClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;
    Ok(token)
}

