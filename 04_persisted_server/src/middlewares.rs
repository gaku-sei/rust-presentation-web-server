use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::{
    headers::{authorization::Basic, Authorization},
    TypedHeader,
};

static ALLOWED_USERNAME: &str = "admin";
static ALLOWED_PASSWORD: &str = "admin";

use crate::errors::Result;

pub async fn auth(
    TypedHeader(auth): TypedHeader<Authorization<Basic>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Obviously not production code
    if auth.username() != ALLOWED_USERNAME || auth.password() != ALLOWED_PASSWORD {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
