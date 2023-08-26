use anyhow::Result;
use async_graphql::Error;
use entity::entities::image;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use super::{super::types::GeneratedImage, uploader::Uploader};

pub struct ImageManger;

impl ImageManger {
    pub async fn insert_generated(
        db: &DatabaseConnection,
        image_data: GeneratedImage,
        user_id: Option<i32>,
    ) -> Result<image::Model, Error> {
        let upload_data = Uploader::new()
            .upload_image(image_data.url.clone(), None)
            .await?;
        let new_image = image::ActiveModel {
            url: Set(upload_data.secure_url),
            public_id: Set(Some(upload_data.public_id)),
            title: Set(Some(image_data.prompt)),
            model: Set(Some(image_data.model)),
            user_id: Set(user_id),
            ..Default::default()
        };
        let new_image = new_image.insert(db).await?;
        return Ok(new_image);
    }

    pub async fn insert_many_generated(
        db: &DatabaseConnection,
        image_data: Vec<GeneratedImage>,
        user_id: Option<i32>,
    ) -> Result<Vec<image::Model>, Error> {
        let mut new_images: Vec<image::Model> = vec![];
        for image in image_data {
            new_images.push(ImageManger::insert_generated(db, image, user_id).await?)
        }
        Ok(new_images)
    }

    pub async fn get_image_by_id(db: &DatabaseConnection, id: i32) -> Result<image::Model> {
        let image = image::Entity::find_by_id(id).one(db).await?;
        match image {
            Some(image) => Ok(image),
            None => {
                let err_msg = String::from("Specified image was not found");
                Err(DbErr::Custom(err_msg).into())
            }
        }
    }
}
