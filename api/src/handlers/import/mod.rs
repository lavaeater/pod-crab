use crate::handlers::auth::login_required_middleware::login_required_middleware;
use crate::handlers::auth::required_role_middleware::RequiredRoleMiddleware;
use crate::{AppState, PaginationParams};
use poem::error::InternalServerError;
use poem::web::{Data, Html, Multipart, Query};
use poem::{get, handler, post, EndpointExt, IntoResponse, Route};

#[handler]
pub async fn index(
    state: Data<&AppState>,
    Query(_params): Query<PaginationParams>,
) -> poem::Result<impl IntoResponse> {
    let mut ctx = tera::Context::new();

    let body = state
        .templates
        .render("import/index.html.tera", &ctx)
        .map_err(InternalServerError)?;
    Ok(Html(body))
}

#[handler]
async fn upload(mut multipart: Multipart) -> poem::Result<impl IntoResponse> {
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().map(ToString::to_string);
        let file_name = field.file_name().map(ToString::to_string);
        if let Ok(bytes) = field.bytes().await {
            println!(
                "name={:?} filename={:?} length={}",
                name,
                file_name,
                bytes.len()
            );
        }
    }
    Ok(Html("ok"))
}

pub fn import_routes() -> Route {
    Route::new()
        .at("/", get(index).around(login_required_middleware))
        .at(
            "/upload",
            post(upload).with(RequiredRoleMiddleware::new("super_admin")),
        )
}
