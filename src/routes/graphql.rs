use actix_web::{ get, post, web::Data, HttpRequest, HttpResponse, Result};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use crate::{graphql::schema::AppSchema, routes::utils::get_user_from_headers};

#[get("/graphql")]
async fn playground() -> Result<HttpResponse> {
    let response = HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish());
    return Ok(response);
}

#[post("/graphql")]
async fn execute(
    schema: Data<AppSchema>,
    req: HttpRequest,
    gql: GraphQLRequest,
) -> Result<GraphQLResponse> {
    let headers = req.headers();
    let user = get_user_from_headers(headers)?;
    let request = gql.into_inner().data(user);
    Ok(schema.execute(request).await.into())
}

