use entity::entities::image;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

use super::types::GeneratedImage;

pub struct ImageManger;

impl ImageManger {
    pub async fn insert_generated(
        db: &DatabaseConnection,
        image_data: GeneratedImage,
        user_id: Option<i32>,
    ) -> Result<image::Model, DbErr> {
        let new_image = image::ActiveModel {
            url: Set(image_data.url),
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
    ) -> Result<Vec<image::Model>, DbErr> {
        let mut new_images: Vec<image::Model> = vec![];
        for image in image_data {
            new_images.push(ImageManger::insert_generated(db, image, user_id).await?)
        }
        Ok(new_images)
    }
}
