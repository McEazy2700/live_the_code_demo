use async_graphql::*;

use crate::context::AppContext;

use super::auth::utils::verify_google_token;
use super::{types::User, utils::db_manager::UserManager};

#[derive(Default)]
pub struct UserMutations;

#[Object]
impl UserMutations {
    async fn google_signup(&self, cx: &Context<'_>, id_token: String) -> Result<User, Error> {
        // Todo: Signup and SignIn mix
        let db = &cx.data::<AppContext>()?.db;
        let token_info = verify_google_token(id_token).await?;
        let user = UserManager::insert_user(db, token_info.email, token_info.picture).await?;
        Ok(user.into())
    }
}
