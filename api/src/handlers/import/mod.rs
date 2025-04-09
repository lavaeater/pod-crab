use std::str::FromStr;
use crate::handlers::auth::login_required_middleware::login_required_middleware;
use crate::handlers::auth::required_role_middleware::RequiredRoleMiddleware;
use crate::{AppState, PaginationParams};
use poem::error::InternalServerError;
use poem::http::StatusCode;
use poem::web::{Data, Html, Multipart, Query};
use poem::{get, handler, post, EndpointExt, IntoResponse, Route};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use entities::member;
use service::{calculate_member_hash, MutationCore, QueryCore};

#[handler]
pub async fn index(
    state: Data<&AppState>,
    Query(_params): Query<PaginationParams>,
) -> poem::Result<impl IntoResponse> {
    let mut ctx = tera::Context::new();

    let imports = QueryCore::list_imports(&state.conn)
        .await
        .map_err(InternalServerError)?;
    ctx.insert("imports", &imports);
    
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
pub async fn upload(state: Data<&AppState>, mut multipart: Multipart) -> poem::Result<impl IntoResponse> {
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
                            _ => return Ok(StatusCode::ACCEPTED.with_header("HX-Redirect", "/import")),
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
                    let mut csv_reader = csv::Reader::from_reader(bytes.as_slice());
                    let conn = &state.conn;
                    let mut _imported = 0;
                    let mut _skipped = 0;
                    
                    for r in csv_reader.records() {
                        let record = r.map_err(InternalServerError)?;
                        let first_name = record.get(0).unwrap();
                        let last_name = record.get(1).unwrap();
                        let birthdate = record.get(2).unwrap();
                        let phone_number = record.get(3).unwrap();
                        let email = record.get(4).unwrap();
                        
                        // Calculate hash for the record
                        let birth_date = sea_orm::prelude::Date::from_str(birthdate).unwrap_or_else(|_| (sea_orm::prelude::Date::MIN));
                        let record_hash = calculate_member_hash(first_name, last_name, &birth_date, phone_number, email);
                        
                        // Check if member with similar data already exists
                        if member_exists_by_hash(conn, &record_hash).await {
                            _skipped += 1;
                            continue;
                        }
                        
                        // Create the member
                        let member_model = member::Model {
                            id: sea_orm::prelude::Uuid::new_v4(),
                            first_name: first_name.to_string(),
                            last_name: last_name.to_string(),
                            birth_date,
                            mobile_phone: phone_number.to_string(),
                            email: email.to_string(),
                            hash: record_hash,
                        };
                        
                        if let Err(e) = MutationCore::create_member(conn, member_model).await {
                            // Handle error (log it, but continue processing other records)
                            log::error!("Failed to create member: {}", e);
                        } else {
                            _imported += 1;
                        }
                    }
                    Ok(StatusCode::ACCEPTED.with_header("HX-Redirect", "/import"))
                }
                ImportType::Transactions => {
                    let mut _cursor = std::io::Cursor::new(&bytes);
                    Ok(StatusCode::ACCEPTED.with_header("HX-Redirect", "/import"))
                }
            }
        }
        _ => Ok(StatusCode::ACCEPTED.with_header("HX-Redirect", "/import")),
    }
}

/// Check if a member with similar data already exists in the database
async fn member_exists_by_hash(
    conn: &sea_orm::DatabaseConnection,
    hash: &str
) -> bool {
    // Check for existing members with the same email (primary check)
    let hash_match = member::Entity::find()
        .filter(member::Column::Hash.eq(hash.to_string()))
        .one(conn)
        .await;

    match hash_match {
        Ok(m) => {
            match m {
                Some(_) => {
                    true
                }
                None => {
                    false
                }
            }
        }
        Err(_) => {
            false
        }
    }
}

/// Check if a member with similar data already exists in the database
#[allow(dead_code)]
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
