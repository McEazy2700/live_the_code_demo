use std::env::var;
use serde::{Deserialize, Serialize};
use reqwest::{self, Error, header::{HeaderMap, HeaderValue}};

use super::ai_models::AIModel;

#[derive(Deserialize, Serialize, Debug)]
pub struct ImagePrompt {
    pub model: String,
    pub prompt: String,
    pub size: String,
    pub n: i32,
    pub response_format: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ImageUrl  {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct  ImageGenerationResponse {
    pub data: Vec<ImageUrl>,
    pub model: String
}

pub struct Generator {
    api_key: String,
    api_url: String,
}

pub enum ImageSize {
    Small,
    Medium,
    Large,
}

impl ImageSize {
    fn value(&self) -> String {
        match *self {
            ImageSize::Small => String::from("256x256"),
            ImageSize::Medium => String::from("512x512"),
            ImageSize::Large => String::from("1024x1024")
        }
    }
}

impl Default for ImageSize {
    fn default() -> Self {
        ImageSize::Medium
    }
}

pub struct GeneratedImage {
    pub prompt: String,
    pub url: String,
    pub model: String,
}

impl GeneratedImage {
    pub fn new(prompt: &str, url: &str, model: &str) -> Self {
        Self { prompt: String::from(prompt), url: String::from(url), model: String::from(model) }
    }
}

impl Generator {
    pub fn new() -> Self {
        Self {
            api_key: var("CHIMERA_API_KEY").expect("Extract CHIMERA_API_KEY env var"),
            api_url: String::from("https://chimeragpt.adventblocks.cc/api/v1/images/generations"),
        }
    }

    pub async fn generate_image(
        &self,
        prompt: String,
        model: Option<AIModel>,
        size: Option<ImageSize>,
        number: Option<i32>
    ) -> Result<Vec<GeneratedImage>, Error> {
        let model = match model {
            Some(model) => model,
            None => AIModel::default(),
        };

        let size = match size {
            Some(size) => size.value(),
            None => ImageSize::default().value(),
        };
        let number = match number {
            Some(num) => if num <= model.max_images {num} else {model.max_images},
            None => 1,
        };

        let image_prompt = ImagePrompt {
            size,
            prompt: prompt.clone(),
            model: model.id,
            response_format: String::from("url"),
            n: number,
        };
        
            
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        
        let token = format!("Bearer {}", &self.api_key);
        headers.insert("Authorization", HeaderValue::from_str(token.as_str()).unwrap());
        headers.insert("Content-type", HeaderValue::from_str("application/json").unwrap());
        
        match client.post(&self.api_url)
                    .headers(headers)
                    .json(&image_prompt)
                    .send()
                    .await?
                    .error_for_status() {
            Ok(val) => {
                let response: ImageGenerationResponse = val.json().await?;
                let mut generated: Vec<GeneratedImage> = vec![];
                for image_data in response.data {
                    generated.push(
                        GeneratedImage::new(
                            prompt.as_str(),
                            image_data.url.as_str(),
                            response.model.as_str()
                        )
                    )
                }
                Ok(generated)
            },
            Err(err) => Err(err),
        }
    }
}
