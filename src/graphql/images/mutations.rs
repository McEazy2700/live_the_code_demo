use async_graphql::*;
use crate::graphql::images::generator::Generator;

use super::types::Image;

#[derive(Default)]
pub struct ImageMutations;

#[Object]
impl ImageMutations {
    async fn generate_images(&self, _cx: &Context<'_>, prompt: String, number: Option<i32>) -> Result<Vec<Image>, Error> {
        let generator = Generator::new();
        let generated = generator.generate_image(prompt, None, None, number).await?;
        let mut images: Vec<Image> = vec![];
        let mut id = 1;
        generated.iter().for_each(|val| {
            images.push(Image {
                id,
                title: Some(val.prompt.clone()),
                url: val.url.clone(),
                date_added: String::from("now"),
                user_id: id,
                model: Some(val.model.clone())
            });
            id += 1;
        });
        return Ok(images)
    }
}
