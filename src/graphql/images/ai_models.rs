pub struct AIModel {
    pub id: String,
    pub owned_by: String,
    pub max_images: i32
}

impl Default for AIModel {
    fn default() -> Self {
        ImageModels::kandinsky_2_2()
    }
}

pub struct ImageModels;


impl ImageModels {
    pub fn sdlx() -> AIModel {
        AIModel {
          id: String::from("sdxl"),
          owned_by: String::from("stabilityai"),
          max_images: 5
        } 
    }
    
    pub fn kandinsky_2_2() -> AIModel {
        AIModel {
          id: String::from("kandinsky-2.2"),
          owned_by: String::from("sberbank"),
          max_images: 10
        }
    }

    pub fn kandinsky_2() -> AIModel {
        AIModel{
          id: String::from("kandinsky-2"),
          owned_by: String::from("sberbank"),
          max_images: 10
        }
    }

    pub fn dall_e() -> AIModel {
        AIModel {
          id: String::from("dall-e"),
          owned_by: String::from("openai"),
          max_images: 10
        }
    }

    pub fn stable_diffusion_2_1() -> AIModel {
        AIModel {
          id: String::from("stable-diffusion-2.1"),
          owned_by: String::from("stabilityai"),
          max_images: 10
        }
    }

    pub fn stable_diffusion_2_5() -> AIModel {
        AIModel {
          id: String::from("stable-diffusion-1.5"),
          owned_by: String::from("stabilityai"),
          max_images: 10
        }
    }

    pub fn deepfloyd_if() -> AIModel {
        AIModel {
          id: String::from("deepfloyd-if"),
          owned_by: String::from("deepfloyd"),
          max_images: 4
        }
    }

    pub fn material_diffusion() -> AIModel {
        AIModel {
          id: String::from("material-diffusion"),
          owned_by: String::from("yes"),
          max_images: 8
        }
    }
}

