// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::handlers::auth::login_required_middleware::login_required_middleware;
use crate::handlers::auth::required_role_middleware::RequiredRoleMiddleware;
use crate::{AppState, PaginationParams, DEFAULT_ITEMS_PER_PAGE};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use entities::episode::Model as Episode;
use entities::post;
use oauth2::http::StatusCode;
use poem::error::InternalServerError;
use poem::web::{Data, Form, Html, Path, Query};
use poem::{get, handler, post, EndpointExt, IntoResponse, Route};
use sea_orm::prelude::Uuid;
use service::{Mutation as MutationCore, Query as QueryCore};
use std::error::Error;
use std::time::Duration;

#[derive(Debug)]
struct Opt {
    /// The AWS Region.
    region: Option<String>,

    /// The name of the bucket.
    bucket: String,

    /// The object key.
    object: String,

    /// How long in seconds before the presigned request should expire.
    expires_in: Option<u64>,

    /// Whether to display additional information.
    verbose: bool,
}

async fn get_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<(), Box<dyn Error>> {
    let expires_in = Duration::from_secs(expires_in);
    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    println!("Object URI: {}", presigned_request.uri());
    let valid_until = chrono::offset::Local::now() + expires_in;
    println!("Valid until: {valid_until}");

    Ok(())
}

async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    Ok(())

    // let region_provider = RegionProviderChain::first_try(region.map(Region::new))
    //     .or_default_provider()
    //     .or_else(Region::new("us-west-2"));
    //
    // let shared_config = aws_config::from_env().region(region_provider).load().await;
    // let client = Client::new(&shared_config);

    // get_object(&client, &bucket, &object, expires_in.unwrap_or(900)).await
}

#[handler]
pub async fn create(
    state: Data<&AppState>,
    form: Form<Episode>,
) -> poem::Result<impl IntoResponse> {
    let form = form.0;
    let conn = &state.conn;

    MutationCore::create_episode(conn, form)
        .await
        .map_err(InternalServerError)?;

    Ok(StatusCode::ACCEPTED.with_header("HX-Redirect", "/posts"))
}

#[handler]
pub async fn list(
    state: Data<&AppState>,
    Query(params): Query<PaginationParams>,
) -> poem::Result<impl IntoResponse> {
    let conn = &state.conn;
    let page = params.page.unwrap_or(1);
    let items_per_page = params.items_per_page.unwrap_or(DEFAULT_ITEMS_PER_PAGE);

    let (episodes, num_pages) = QueryCore::find_episodes(conn, page, items_per_page)
        .await
        .map_err(InternalServerError)?;

    let mut ctx = tera::Context::new();
    ctx.insert("episodes", &episodes);
    ctx.insert("page", &page);
    ctx.insert("items_per_page", &items_per_page);
    ctx.insert("num_pages", &num_pages);

    let body = state
        .templates
        .render("episodes/list.html.tera", &ctx)
        .map_err(InternalServerError)?;
    Ok(Html(body))
}

#[handler]
pub async fn new(state: Data<&AppState>) -> poem::Result<impl IntoResponse> {
    let ctx = tera::Context::new();
    let body = state
        .templates
        .render("episodes/new.html.tera", &ctx)
        .map_err(InternalServerError)?;
    Ok(Html(body))
}

#[handler]
pub async fn edit(state: Data<&AppState>, Path(id): Path<Uuid>) -> poem::Result<impl IntoResponse> {
    let conn = &state.conn;

    let post: post::Model = QueryCore::find_post_by_id(conn, id)
        .await
        .map_err(InternalServerError)?
        .ok_or_else(|| poem::Error::from_status(StatusCode::NOT_FOUND))?;

    let mut ctx = tera::Context::new();
    ctx.insert("post", &post);

    let body = state
        .templates
        .render("posts/edit.html.tera", &ctx)
        .map_err(InternalServerError)?;
    Ok(Html(body))
}

#[handler]
pub async fn update(
    state: Data<&AppState>,
    Path(id): Path<Uuid>,
    form: Form<post::Model>,
) -> poem::Result<impl IntoResponse> {
    let conn = &state.conn;
    let form = form.0;

    let post = MutationCore::update_post_by_id(conn, id, form)
        .await
        .map_err(InternalServerError)?;

    let mut ctx = tera::Context::new();
    ctx.insert("post", &post);

    let body = state
        .templates
        .render("posts/post_row.html.tera", &ctx)
        .map_err(InternalServerError)?;
    Ok(Html(body))
}

#[handler]
pub async fn destroy(
    state: Data<&AppState>,
    Path(id): Path<Uuid>,
) -> poem::Result<impl IntoResponse> {
    let conn = &state.conn;

    MutationCore::delete_post(conn, id)
        .await
        .map_err(InternalServerError)?;

    Ok(StatusCode::ACCEPTED.with_header("HX-Redirect", "/posts"))
}

pub fn episode_routes() -> Route {
    Route::new()
        .at("/", get(list).around(login_required_middleware))
        .at(
            "/create",
            post(create).with(RequiredRoleMiddleware::new("super_admin")),
        )
        .at(
            "/new",
            get(new).with(RequiredRoleMiddleware::new("super_admin")),
        )
        .at(
            "/:id",
            get(edit)
                .patch(update)
                .delete(destroy)
                .with(RequiredRoleMiddleware::new("super_admin")),
        )
}
