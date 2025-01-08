use std::env;
use poem::{get, EndpointExt, Route, Server};
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;
use tera::Tera;
use migration::{Migrator, MigratorTrait};
use crate::handlers::{index, members, posts};

mod handlers;


const DEFAULT_ITEMS_PER_PAGE: u64 = 5;

#[derive(Debug, Clone)]
struct AppState {
    templates: Tera,
    conn: DatabaseConnection,
}

#[derive(Deserialize, Default)]
struct PaginationParams {
    page: Option<u64>,
    items_per_page: Option<u64>,
}

#[tokio::main]
async fn start(root_path: Option<String>) -> std::io::Result<()> {
    let root_path = if let Some(root_path) = root_path { root_path } else { env::current_dir()?.to_str().unwrap().to_string() };
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // create post table if not exists
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let templates = Tera::new(&format!("{}/templates/**/*", &root_path)).unwrap();
    let state = AppState { templates, conn };

    println!("Starting server at {server_url}");

    let app = Route::new()
        .at("/", get(index::index))
        .nest("/posts", posts::routes())
        .nest("/members", members::routes())
        .nest(
            "/static",
            StaticFilesEndpoint::new(format!("{}/static", &root_path)),
        )
        .nest(
            "/dist",
            StaticFilesEndpoint::new(format!("{}/dist", &root_path)),
        )
        .data(state);
    let server = Server::new(TcpListener::bind(format!("{host}:{port}")));
    server.run(app).await
}

pub fn main(root_path: Option<String>) {
    let result = start(root_path);

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
