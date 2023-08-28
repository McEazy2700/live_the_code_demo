use actix_web::{
    get,
    middleware::Logger,
    web::{Data, Path},
    App, HttpRequest, HttpResponse, HttpServer,
};
use actix_cors::Cors;
use std::env::var;

use context::AppContext;
use dotenv::dotenv;
use graphql::schema::build_schema;
use migration::{config::DB, Migrator, MigratorTrait};
use routes::graphql::{execute, playground};
pub mod context;
pub mod graphql;
pub mod routes;

#[get("/hello/{name}")]
async fn hello(path: Path<String>, req: HttpRequest) -> HttpResponse {
    let name = path.into_inner();
    let path = req.path();
    let body = format!("Hello {name} from {path}");
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // env_logger::Builder::from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    // Connect to database and migrate
    let db = DB::init();
    let conn = db.connect().await.expect("Datbase Connection Failed");
    Migrator::up(&conn, None).await.expect("Migration failed");

    // build AppContext and Graphql Schema
    let context = AppContext::new(conn);
    let schema = build_schema(context.clone());
    let url = build_url();
    println!("Server started on {url}");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(context.clone()))
            .service(hello)
            .service(playground)
            .service(execute)
    })
    .bind(url)?
    .run()
    .await
}

fn build_url() -> String {
    let debug = var("DEBUG").unwrap_or(String::from("false")) == "true";
    if debug {
        String::from(format!("127.0.0.1:{}", 8080))
    } else {
        let port = var("PORT").expect("PORT env var should be set");
        String::from(format!("0.0.0.0:{port}"))
    }
}
