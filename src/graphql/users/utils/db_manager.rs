use anyhow::Result;
use entity::entities::{image, profile, user, token};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, ModelTrait,
};

pub struct UserManager;

impl UserManager {
    pub async fn insert_user(
        db: &DatabaseConnection,
        email: String,
        image_url: String,
    ) -> Result<user::Model> {
        // Create User
        let new_user = user::ActiveModel {
            email: Set(email),
            ..Default::default()
        };
        let new_user = new_user.insert(db).await?;

        // Create Profile Image if image_url
        let profile_image = image::ActiveModel {
            url: Set(image_url),
            user_id: Set(Some(new_user.id)),
            ..Default::default()
        };
        let profile_image = profile_image.insert(db).await?;

        // Create Profile
        let user_profile = profile::ActiveModel {
            user_id: Set(new_user.id),
            image_id: Set(Some(profile_image.id)),
            ..Default::default()
        };

        user_profile.insert(db).await?;
        return Ok(new_user);
    }

    pub async fn get_user_by_id(db: &DatabaseConnection, id: i32) -> Result<user::Model> {
        let user = user::Entity::find_by_id(id).one(db).await?;
        match user {
            Some(user) => Ok(user),
            None => {
                let err_msg = String::from("User with that id was not found");
                Err(DbErr::Custom(err_msg).into())
            }
        }
    }

    /// Trys to find a user by their email, and returns an Option<user::Model> indicating the user might not exist
    pub async fn try_get_user_by_email(
        db: &DatabaseConnection,
        email: String,
    ) -> Result<Option<user::Model>> {
        let user = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?;
        Ok(user)
    }

    /// Gets and returs a user by their email, and returns an error if not found
    pub async fn get_user_by_email(db: &DatabaseConnection, email: String) -> Result<user::Model> {
        let user = Self::try_get_user_by_email(db, email).await?;
        match user {
            Some(user) => Ok(user),
            None => {
                let err_msg = String::from("User with that email was not found");
                Err(DbErr::Custom(err_msg).into())
            }
        }
    }

    pub async fn get_profile_from_user(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<profile::Model> {
        let profile = profile::Entity::find()
            .filter(profile::Column::UserId.eq(user_id))
            .one(db)
            .await?;
        match profile {
            Some(profile) => Ok(profile),
            None => {
                let err_msg = String::from("Profile for this user was not found");
                Err(DbErr::Custom(err_msg).into())
            }
        }
    }

    pub async fn save_user_token(db: &DatabaseConnection, token: String, user_id: i32) -> Result<token::Model> {
        let old_token = token::Entity::find().filter(token::Column::UserId.eq(user_id)).one(db).await?;
        if let Some(old_token) = old_token {
            old_token.delete(db).await?;
        };
        let new_token = token::ActiveModel {
            user_id: Set(user_id),
            token: Set(token),
            ..Default::default()
        };
        let new_token = new_token.insert(db).await?;
        Ok(new_token)
    }
}
