use async_graphql::*;

#[derive(SimpleObject)]
pub struct Image {
    pub id: i32,
    pub title: Option<String>,
    pub url: String,
    pub date_added: String,
    pub user_id: i32,
    pub model: Option<String>
}

