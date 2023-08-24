use async_graphql::*;

use crate::context::AppContext;

use super::users::types::User;

pub struct Query;

#[Object]
impl Query {
    async fn version(&self) -> String {
        String::from("0.0.1")
    }

    async fn me(&self, cx: &Context<'_>) -> Result<User> {
        let user = &cx.data::<Option<User>>()?;
        match user {
            Some(user) => Ok(user.clone()),
            None => Err(Error::new("Anonymous user")),
        }
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema(context: AppContext) -> AppSchema {
    AppSchema::build(Query, EmptyMutation, EmptySubscription)
        .data(context)
        .finish()
}
