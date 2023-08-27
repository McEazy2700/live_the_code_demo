use async_graphql::*;
use entity::entities::user;

use crate::context::AppContext;

use super::{super::utils::db_manager::UserManager, profile::Profile};


#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub email: String,
}

impl User {
    pub fn new(id: i32, email: String) -> Self {
        Self { id, email }
    }
}

#[ComplexObject]
impl User {
    async fn profile(&self, cx: &Context<'_>) -> Result<Profile, Error> {
        let db = &cx.data::<AppContext>()?.db;
        let profile = UserManager::get_profile_from_user(db, self.id).await?;
        Ok(profile.into())
    }
}

impl From<user::Model> for User {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id,
            email: value.email,
        }
    }
}
