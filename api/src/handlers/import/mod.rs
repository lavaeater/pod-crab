use poem::{handler, IntoResponse};
use poem::error::InternalServerError;
use poem::web::{Data, Html, Query, Multipart};
use crate::{AppState, PaginationParams};

#[handler]
async fn upload(mut multipart: Multipart) -> poem::Result<impl IntoResponse>  {
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
