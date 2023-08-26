use async_graphql::Error;
use cloudinary::{upload::{UploadOptions, result::{UploadResult, Response}}, Cloudinary};
use anyhow::Result;
use std::env::var;

pub struct Uploader {
    client: Cloudinary,
}

impl Uploader {
    pub fn new() -> Self {
        let api_key = var("CLOUDINARY_API_KEY").expect("CLOUDINARY_API_KEY");
        let cloud_name = var("CLOUDINARY_CLOUD_NAME").expect("CLOUDINARY_CLOUD_NAME");
        let api_secret = var("CLOUDINARY_API_SECRET").expect("CLOUDINARY_API_SECRET");

        let uploader = Cloudinary::new(api_key, cloud_name, api_secret);
        return Self { client: uploader }
    }

    pub async fn upload_image(&self, image_url: String, public_id: Option<String>) -> Result<Box<Response>, Error> {
        let options = UploadOptions::new();
        let options = if let Some(public_id) = public_id {
            options.set_public_id(public_id)
        } else {
            options
        };

        let response = self.client
            .upload_image_from_url(image_url.clone(), &options)
            .await?;
        
        let response = match response {
            UploadResult::Success(val) => Ok(val),
            UploadResult::Error(err) => Err(Error::new(err.error.message))
        };
        return response
    }
}
