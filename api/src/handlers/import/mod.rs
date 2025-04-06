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

#[derive(Debug)]
enum ImportType {
    Members,
    Transactions,
}

#[handler]
async fn upload(mut multipart: Multipart) -> poem::Result<impl IntoResponse> {
    let mut import_type = None;
    let mut file_data = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        if let Some(name) = field.name() {
            match name {
                "import_type" => {
                    if let Ok(value) = field.text().await {
                        import_type = Some(match value.as_str() {
                            "members" => ImportType::Members,
                            "transactions" => ImportType::Transactions,
                            _ => return Ok(Html("Invalid import type")),
                        });
                    }
                }
                "file" => {
                    if let Ok(bytes) = field.bytes().await {
                        file_data = Some(bytes);
                    }
                }
                _ => {}
            }
        }
    }

    match (import_type, file_data) {
        (Some(import_type), Some(bytes)) => {
            println!(
                "Processing {:?} import with file size {}",
                import_type,
                bytes.len()
            );
            // TODO: Process the file based on import_type
            Ok(Html("Import successful"))
        }
        _ => Ok(Html("Missing import type or file")),
    }
}

pub fn import_routes() -> Route {
    Route::new()
        .at("/", get(index).around(login_required_middleware))
        .at(
            "/upload",
            post(upload).around(RequiredRoleMiddleware::new(vec!["admin"])),
        )
}
