use crate::handlers::auth::{auth_middleware, setup_openid_client, GoogleClient};
use crate::handlers::{auth, index, members, posts};
use migration::{Migrator, MigratorTrait};
use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::{get, EndpointExt, Route, Server};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait};
use serde::Deserialize;
use std::env;
use std::str::FromStr;
use openidconnect::Nonce;
use poem::session::{CookieConfig, CookieSession};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Uuid;
use tera::Tera;
use entities::prelude::User;
use entities::user;

mod handlers;


const DEFAULT_ITEMS_PER_PAGE: u64 = 5;

#[derive(Debug, Clone)]
struct AppState {
    templates: Tera,
    conn: DatabaseConnection
}

#[derive(Debug, Clone)]
struct OpenIdData {
    google_client: GoogleClient,
    nonce: Option<Nonce>,
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
    
    ensure_super_admin(&conn).await;
    
    let templates = Tera::new(&format!("{}/templates/**/*", &root_path)).unwrap();
    let google_client = setup_openid_client().await.unwrap();
    let state = AppState { templates, conn };
    let open_id_data = OpenIdData {
        google_client,
        nonce: None,
    };

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
        .with(CookieSession::new(CookieConfig::default())) //.secure(true)
        .data(state)
        .data(open_id_data);
    let server = Server::new(TcpListener::bind(format!("{host}:{port}")));
    server.run(app).await
}

async fn ensure_super_admin(database_connection: &DatabaseConnection) {
    let user_id = Uuid::from_str("920b2fc5-d127-4003-b3f9-43bb685558d4").unwrap();
    if let Ok(Some(_user)) = user::Entity::find_by_id(user_id.clone()).one(database_connection).await {
        return;
    }
    
    let _u = user::ActiveModel {
        id: Set(user_id),
        email: Set("tommie.nygren@gmail.com".to_string()),
        name: Set("Tommie Nygren".to_string()),
        role: Set("super_admin".to_string()),
    }
        .insert(database_connection)
        .await;
}

pub fn main(root_path: Option<String>) {
    let result = start(root_path);

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
