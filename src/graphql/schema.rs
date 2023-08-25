use async_graphql::*;

use crate::context::AppContext;

use super::{users::types::User, images::mutations::ImageMutations};

pub struct Query;


#[derive(MergedObject, Default)]
pub struct Mutation(ImageMutations);

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

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(context: AppContext) -> AppSchema {
    AppSchema::build(Query, Mutation::default(), EmptySubscription)
        .data(context)
        .finish()
}
