use crate::graphql::users::auth::utils::ensure_auth;
use crate::{context::AppContext, graphql::images::generator::Generator};
use async_graphql::*;

use super::db_manager::ImageManger;
use super::types::{GeneratedImage, Image, ImageGenerationInput};

#[derive(Default)]
pub struct ImageMutations;

#[Object]
impl ImageMutations {
    async fn generate_images(
        &self,
        input: ImageGenerationInput,
    ) -> Result<Vec<GeneratedImage>, Error> {
        let generator = Generator::new();
        let model = match input.model {
            Some(model) => Some(model.value()),
            None => None,
        };

        let generated = generator
            .generate_images(input.prompt, model, input.size, input.number)
            .await?;
        return Ok(generated);
    }

    async fn save_generated_images(
        &self,
        cx: &Context<'_>,
        images: Vec<GeneratedImage>,
    ) -> Result<Vec<Image>, Error> {
        let db = &cx.data::<AppContext>()?.db;
        let user = ensure_auth(cx)?;
        let saved = ImageManger::insert_many_generated(db, images, Some(user.id)).await?;
        let mut images: Vec<Image> = vec![];
        saved.iter().for_each(|img| images.push(img.into()));
        Ok(images)
    }
}
