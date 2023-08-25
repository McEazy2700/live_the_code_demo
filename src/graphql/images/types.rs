use async_graphql::*;
use entity::entities::image;

use super::utils::ai_models::{AIModel, ImageModels};

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "SaveGeneratedImageInput")]
pub struct GeneratedImage {
    pub prompt: String,
    pub url: String,
    pub model: String,
}

impl GeneratedImage {
    pub fn new(prompt: &str, url: &str, model: &str) -> Self {
        Self {
            prompt: String::from(prompt),
            url: String::from(url),
            model: String::from(model),
        }
    }
}

#[derive(InputObject)]
pub struct ImageGenerationInput {
    pub prompt: String,
    pub model: Option<AIModels>,
    pub number: Option<i32>,
    pub size: Option<ImageSize>,
}

#[derive(SimpleObject)]
pub struct Image {
    pub id: i32,
    pub title: Option<String>,
    pub url: String,
    pub date_added: String,
    pub user_id: Option<i32>,
    pub model: Option<String>,
}

impl From<&image::Model> for Image {
    fn from(value: &image::Model) -> Self {
        Self {
            id: value.id,
            title: value.title.clone(),
            url: value.url.clone(),
            date_added: value.date_added.to_string(),
            user_id: value.user_id,
            model: value.model.clone(),
        }
    }
}

#[derive(Enum, Clone, Copy, Eq, PartialEq)]
pub enum ImageSize {
    Small,
    Medium,
    Large,
}

impl ImageSize {
    pub fn value(&self) -> String {
        match *self {
            ImageSize::Small => String::from("256x256"),
            ImageSize::Medium => String::from("512x512"),
            ImageSize::Large => String::from("1024x1024"),
        }
    }
}

impl Default for ImageSize {
    fn default() -> Self {
        ImageSize::Medium
    }
}

#[derive(Enum, Clone, Copy, Eq, PartialEq)]
pub enum AIModels {
    SDLX,
    Kandinsky2_2,
    Kandinsky2,
    DallE,
    StableDiffusion2_1,
    StableDiffusion2_5,
    DeepfloydIf,
    MaterialDiffusion,
}

impl AIModels {
    pub fn value(&self) -> AIModel {
        match *self {
            AIModels::SDLX => ImageModels::sdlx(),
            AIModels::DallE => ImageModels::dall_e(),
            AIModels::Kandinsky2 => ImageModels::kandinsky_2(),
            AIModels::Kandinsky2_2 => ImageModels::kandinsky_2_2(),
            AIModels::StableDiffusion2_1 => ImageModels::stable_diffusion_2_1(),
            AIModels::StableDiffusion2_5 => ImageModels::stable_diffusion_2_5(),
            AIModels::MaterialDiffusion => ImageModels::material_diffusion(),
            AIModels::DeepfloydIf => ImageModels::deepfloyd_if(),
        }
    }
}

impl Default for AIModels {
    fn default() -> Self {
        AIModels::Kandinsky2_2
    }
}
