use async_graphql::*;

use crate::graphql::users::types::User;

pub fn ensure_auth(cx: &Context<'_>) -> Result<User, Error> {
    let user = cx.data::<Option<User>>()?;
    match user {
        Some(user) => Ok(user.clone()),
        None => Err(Error::new(
            "Validation Error: You are not allowed to perfom this action",
        )),
    }
}
