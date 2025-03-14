use poem::{Endpoint, IntoResponse, Middleware, Result, Response};
use poem::error::{ NotFoundError};
use poem::Request;
use poem::session::Session;
use poem::web::Redirect;
use entities::user::Model as User;
use crate::handlers::auth::REDIRECT_AFTER_LOGIN_KEY;

pub struct RequiredRoleMiddleware {
    role: String,
}

impl RequiredRoleMiddleware {
    pub fn new(role: &str) -> Self {
        Self { role: role.to_string() }
    }
}

impl<E: Endpoint> Middleware<E> for RequiredRoleMiddleware {
    type Output = RequiredRoleMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        RequiredRoleMiddlewareImpl { role: self.role.clone(), ep }
    }
}

/// The new endpoint type generated by the TokenMiddleware.
pub struct RequiredRoleMiddlewareImpl<E> {
    role: String,
    ep: E,
}

impl<E: Endpoint> Endpoint for RequiredRoleMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let session = req.extensions().get::<Session>();

        if let Some(session) = session {
            // Check if user is logged in and has the required role
            if let Some(user) = session.get::<User>("current_user") {
                return if user.role == self.role {
                    self.ep.call(req).await.map(IntoResponse::into_response)
                } else {
                    Err(NotFoundError.into())
                }
            } else {
                session.set(REDIRECT_AFTER_LOGIN_KEY, req.uri().path().to_string());
            }
        }

        Ok(Redirect::temporary("/auth/login").into_response())
    }
}
