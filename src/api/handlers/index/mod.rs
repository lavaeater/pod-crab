use poem::{handler, IntoResponse};
use poem::error::InternalServerError;
use poem::web::{Data, Html, Query};
use crate::api::{AppState, PaginationParams};

#[handler]
pub async fn index(state: Data<&AppState>, Query(_params): Query<PaginationParams>) -> poem::Result<impl IntoResponse> {
    let mut ctx = tera::Context::new();
    
    let body = state
        .templates
        .render("index.html.tera", &ctx)
        .map_err(InternalServerError)?;
    Ok(Html(body))
}