use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct Error(anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("an error occured: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
