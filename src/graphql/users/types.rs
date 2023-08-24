use async_graphql::*;

#[derive(SimpleObject, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
}

impl User {
    pub fn new(id: i32, email: String) -> Self {
        Self { id, email }
    }
}
