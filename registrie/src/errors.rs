use axum::response::{IntoResponse, Response};

pub type RegistrieResult<T> = Result<T, RegistrieError>;

#[derive(thiserror::Error, Debug)]
pub enum RegistrieError {
    // TODO: Validation of user requests and fields
}

impl IntoResponse for RegistrieError {
    fn into_response(self) -> Response {
        todo!()
    }
}
