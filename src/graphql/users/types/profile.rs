use crate::context::AppContext;
use crate::graphql::images::types::Image;
use crate::graphql::images::utils::db_manager::ImageManger;
use crate::graphql::users::utils::db_manager::UserManager;
use async_graphql::*;
use entity::entities::profile;

use super::user::User;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Profile {
    pub id: i32,
    pub user_name: Option<String>,
    pub bio: Option<String>,

    #[graphql(skip)]
    user_id: i32,

    #[graphql(skip)]
    image_id: Option<i32>,
}

impl From<profile::Model> for Profile {
    fn from(value: profile::Model) -> Self {
        Self {
            id: value.id,
            user_id: value.id,
            user_name: value.user_name,
            bio: value.bio,
            image_id: value.image_id,
        }
    }
}

#[ComplexObject]
impl Profile {
    async fn user(&self, cx: &Context<'_>) -> Result<User, Error> {
        let db = &cx.data::<AppContext>()?.db;
        let user = UserManager::get_user_by_id(db, self.user_id).await?;
        Ok(user.into())
    }

    async fn image(&self, cx: &Context<'_>) -> Result<Option<Image>, Error> {
        let db = &cx.data::<AppContext>()?.db;
        if let Some(image_id) = self.image_id {
            let image = ImageManger::get_image_by_id(db, image_id).await?;
            Ok(Some(image.into()))
        } else {
            Ok(None)
        }
    }
}
