use crate::api::{AppState, PaginationParams, DEFAULT_ITEMS_PER_PAGE};
use crate::entity::member;
use crate::service::{Mutation as MutationCore, Query as QueryCore};
use poem::error::InternalServerError;
use poem::http::StatusCode;
use poem::web::{Data, Form, Html, Path, Query};
use poem::{get, handler, Error, IntoResponse, Route};

#[handler]
pub async fn create(state: Data<&AppState>, form: Form<member::Model>) -> poem::Result<impl IntoResponse> {
    let form = form.0;
    let conn = &state.conn;

    MutationCore::create_member(conn, form)
        .await
        .map_err(InternalServerError)?;

    Ok(StatusCode::ACCEPTED.with_header("HX-Redirect", "/members"))
}

#[handler]
pub async fn list(state: Data<&AppState>, Query(params): Query<PaginationParams>) -> poem::Result<impl IntoResponse> {
    let conn = &state.conn;
    let page = params.page.unwrap_or(1);
    let members_per_page = params.items_per_page.unwrap_or(DEFAULT_ITEMS_PER_PAGE);

    let (members, num_pages) = QueryCore::find_members_in_page(conn, page, members_per_page)
        .await
        .map_err(InternalServerError)?;

    let mut ctx = tera::Context::new();
    ctx.insert("members", &members);
    ctx.insert("page", &page);
    ctx.insert("members_per_page", &members_per_page);
    ctx.insert("num_pages", &num_pages);

    let body = state
        .templates
        .render("members/list.html.tera", &ctx)
        .map_err(InternalServerError)?;
    Ok(Html(body))
}

#[handler]
pub async fn new(state: Data<&AppState>) -> poem::Result<impl IntoResponse> {
    let ctx = tera::Context::new();
    match state
        .templates
        .render("members/new.html.tera", &ctx) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(err) => {
            eprintln!("Tera rendering error: {:?}", err); // Log the error
            Err(InternalServerError(err))
        }
    }
}

#[handler]
pub async fn edit(state: Data<&AppState>, Path(id): Path<i32>) -> poem::Result<impl IntoResponse> {
    let conn = &state.conn;

    let member: member::Model = QueryCore::find_member_by_id(conn, id)
        .await
        .map_err(InternalServerError)?
        .ok_or_else(|| Error::from_status(StatusCode::NOT_FOUND))?;

    let mut ctx = tera::Context::new();
    ctx.insert("member", &member);

    let body = state
        .templates
        .render("members/edit.html.tera", &ctx)
        .map_err(InternalServerError)?;
    Ok(Html(body))
}

#[handler]
pub async fn update(
    state: Data<&AppState>,
    Path(id): Path<i32>,
    form: Form<member::Model>,
) -> poem::Result<impl IntoResponse> {
    let conn = &state.conn;
    let form = form.0;

    let member = MutationCore::update_member_by_id(conn, id, form)
        .await
        .map_err(InternalServerError)?;

    let mut ctx = tera::Context::new();
    ctx.insert("member", &member);

    let body = state
        .templates
        .render("members/member_row.html.tera", &ctx)
        .map_err(InternalServerError)?;
    Ok(Html(body))
}

#[handler]
pub async fn destroy(state: Data<&AppState>, Path(id): Path<i32>) -> poem::Result<impl IntoResponse> {
    let conn = &state.conn;

    MutationCore::delete_member(conn, id)
        .await
        .map_err(InternalServerError)?;

    Ok(StatusCode::ACCEPTED.with_header("HX-Redirect", "/members"))
}

// A function to define all routes related to posts
pub fn routes() -> Route {
    Route::new()
        .at("/", get(list).post(create))
        .at("/new", get(new))
        .at("/:id", get(edit).patch(update).delete(destroy))
}
