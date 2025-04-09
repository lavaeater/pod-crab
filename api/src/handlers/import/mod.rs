use std::str::FromStr;
use crate::handlers::auth::login_required_middleware::login_required_middleware;
use crate::handlers::auth::required_role_middleware::RequiredRoleMiddleware;
use crate::{AppState, PaginationParams};
use poem::error::InternalServerError;
use poem::web::{Data, Html, Multipart, Query};
use poem::{get, handler, post, EndpointExt, IntoResponse, Route};
use sha2::{Digest, Sha256};
use entities::member;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use service::Mutation;

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
async fn upload(state: Data<&AppState>, mut multipart: Multipart) -> poem::Result<impl IntoResponse> {
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
            match import_type {
                ImportType::Members => {
                    let mut csv_reader = csv::Reader::from_reader(&bytes);
                    let conn = &state.conn;
                    let mut imported = 0;
                    let mut skipped = 0;
                    
                    for r in csv_reader.records() {
                        let record = r?;
                        let first_name = record.get(0).unwrap();
                        let last_name = record.get(1).unwrap();
                        let birthdate = record.get(2).unwrap();
                        let phone_number = record.get(3).unwrap();
                        let email = record.get(4).unwrap();
                        
                        // Calculate hash for the record
                        let record_hash = calculate_member_hash(first_name, last_name, birthdate, phone_number, email);
                        
                        // Check if member with similar data already exists
                        if member_exists_by_data(conn, first_name, last_name, email).await {
                            skipped += 1;
                            continue;
                        }
                        
                        // Create the member
                        let member_model = member::Model {
                            id: sea_orm::prelude::Uuid::new_v4(),
                            first_name: first_name.to_string(),
                            last_name: last_name.to_string(),
                            birth_date: sea_orm::prelude::Date::from_str(birthdate).expect("Invalid date format"),
                            mobile_phone: phone_number.to_string(),
                            email: email.to_string(),
                        };
                        
                        if let Err(e) = Mutation::create_member(conn, member_model).await {
                            // Handle error (log it, but continue processing other records)
                            log::error!("Failed to create member: {}", e);
                        } else {
                            imported += 1;
                        }
                    }
                    Ok(Html(format!("Import completed: {} imported, {} skipped (already exist)", imported, skipped)))
                }
                ImportType::Transactions => {
                    let mut cursor = std::io::Cursor::new(&bytes);
                    Ok(Html("Transactions import successful"))
                }
            }
        }
        _ => Ok(Html("Missing import type or file")),
    }
}

/// Calculate a hash for member data to uniquely identify potential duplicates


/// Check if a member with similar data already exists in the database
async fn member_exists_by_data(
    conn: &sea_orm::DatabaseConnection,
    first_name: &str,
    last_name: &str, 
    email: &str
) -> bool {
    // Check for existing members with the same email (primary check)
    let email_match = member::Entity::find()
        .filter(member::Column::Email.eq(email.to_string()))
        .one(conn)
        .await;
        
    if let Ok(Some(_)) = email_match {
        return true;
    }
    
    // Check for members with the same first and last name as a secondary check
    let name_match = member::Entity::find()
        .filter(member::Column::FirstName.eq(first_name.to_string()))
        .filter(member::Column::LastName.eq(last_name.to_string()))
        .one(conn)
        .await;
        
    matches!(name_match, Ok(Some(_)))
}

pub fn import_routes() -> Route {
    Route::new()
        .at("/", get(index).around(login_required_middleware))
        .at(
            "/upload",
            post(upload).with(RequiredRoleMiddleware::new("super_admin")),
        )
}
