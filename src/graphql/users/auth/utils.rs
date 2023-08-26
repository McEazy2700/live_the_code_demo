use anyhow::Result;
use async_graphql::*;

use crate::graphql::users::types::User;

use super::types::GoogleTokenInfo;

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
