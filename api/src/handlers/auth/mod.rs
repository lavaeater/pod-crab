use openidconnect::http::StatusCode;
use poem::{get, handler, Error, IntoResponse, Route};
use poem::error::InternalServerError;
use poem::web::{Data, Form, Html, Path, Query, Redirect};
use sea_orm::prelude::Uuid;
use entities::member;
use crate::{AppState, PaginationParams, DEFAULT_ITEMS_PER_PAGE};

#[handler]
pub async fn login(
    state: Data<&AppState>,
    Query(params): Query<PaginationParams>,
) -> poem::Result<impl IntoResponse> {
    Ok(Html)
    Redirect::moved_permanent("https:// www. google. com")
}

#[handler]
pub async fn new(state: Data<&AppState>) -> poem::Result<impl IntoResponse> {
    let ctx = tera::Context::new();
    match state.templates.render("members/new.html.tera", &ctx) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(err) => {
            eprintln!("Tera rendering error: {:?}", err); // Log the error
            Err(InternalServerError(err))
        }
    }
}

#[handler]
pub async fn edit(state: Data<&AppState>, Path(id): Path<Uuid>) -> poem::Result<impl IntoResponse> {
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
    Path(id): Path<Uuid>,
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

// A function to define all routes related to posts
pub fn routes() -> Route {
    Route::new()
        .at("/", get(list).post(create))
        .at("/new", get(new))
        .at("/:id", get(edit).patch(update).delete(destroy))
}
