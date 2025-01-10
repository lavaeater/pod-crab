use crate::handlers::auth::{auth_middleware, setup_openid_client};
use crate::handlers::{auth, index, members, posts};
use migration::{Migrator, MigratorTrait};
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::{get, EndpointExt, Route, Server};
use sea_orm::{Database, DatabaseConnection};
use serde::Deserialize;
use std::env;
use tera::Tera;

mod handlers;


const DEFAULT_ITEMS_PER_PAGE: u64 = 5;

#[derive(Debug, Clone)]
struct AppState {
    templates: Tera,
    conn: DatabaseConnection
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
    let google_client = setup_openid_client().await.unwrap();
    let state = AppState { templates, conn};

    println!("Starting server at {server_url}");
    let app = Route::new()
        .at("/", get(index::index))
        .nest("/posts", posts::routes().around(auth_middleware))
        .nest("/members", members::routes().around(auth_middleware))
        .nest("/auth", auth::routes())
        .nest(
            "/static",
            StaticFilesEndpoint::new(format!("{}/static", &root_path)),
        )
        .nest(
            "/dist",
            StaticFilesEndpoint::new(format!("{}/dist", &root_path)),
        )
        .data(state)
        .data(google_client);
    let server = Server::new(TcpListener::bind(format!("{host}:{port}")));
    server.run(app).await
}

pub fn main(root_path: Option<String>) {
    let result = start(root_path);

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
