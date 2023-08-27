use async_graphql::*;

use crate::context::AppContext;
use crate::graphql::common::types::Success;

use super::auth::utils::{generate_token, verify_google_token};
use super::types::token::Token;
use super::{types::user::User, utils::db_manager::UserManager};

#[derive(Default)]
pub struct UserMutations;

#[Object]
impl UserMutations {
    /// Signs in a user if they already exist, and registrers them if they do not
    async fn google_auth(&self, cx: &Context<'_>, id_token: String) -> Result<Success<Token>, Error> {
        let db = &cx.data::<AppContext>()?.db;
        let token_info = verify_google_token(id_token).await?;
        let existing_user =
            UserManager::try_get_user_by_email(db, token_info.email.clone()).await?;
        let user: User = match existing_user {
            Some(user) => user.into(),
            None => {
                let user =
                    UserManager::insert_user(db, token_info.email, token_info.picture).await?;
                user.into()
            }
        };
        let token = generate_token(&user)?;
        let token = UserManager::save_user_token(db, token, user.id).await?;
        Ok(Success::<Token>::new(token.into()))
    }
}
