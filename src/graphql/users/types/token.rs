use async_graphql::*;
use crate::{graphql::users::utils::db_manager::UserManager, context::AppContext};
use super::user::User;
use entity::entities::token;


#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
    #[graphql(skip)]
    user_id: i32
}

#[ComplexObject]
impl Token {
    async fn user(&self, cx: &Context<'_>) -> Result<User, Error> {
        let db = &cx.data::<AppContext>()?.db;
        let user = UserManager::get_user_by_id(db, self.user_id).await?;
        Ok(user.into())
    }
}

impl From<token::Model> for Token {
    fn from(value: token::Model) -> Self {
        Self {
            token: value.token,
            refresh_token: value.id.to_string(),
            user_id: value.user_id
        }
    }
}
