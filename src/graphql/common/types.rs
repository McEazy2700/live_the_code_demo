use async_graphql::*;
use crate::graphql::users::types::token::Token;

#[derive(SimpleObject)]
#[graphql(concrete(name = "SuccessToken", params(Token)))]
pub struct Success<T: OutputType> {
    data: Option<T>,
    success: bool,
}

impl<T: OutputType> Success<T> {
    pub fn new(data: T) -> Success<T> {
        Success::<T> {
            data: Some(data),
            success: true
        }
    }
}
