use axum::response::{IntoResponse, Response};

pub type RegistrieResult<T> = Result<T, RegistrieError>;

#[derive(thiserror::Error, Debug)]
pub enum RegistrieError {
    // TODO: Validation of user requests and fields
    // Issue URL: https://github.com/Colabie/Colabie/issues/9
}

impl IntoResponse for RegistrieError {
    fn into_response(self) -> Response {
        todo!()
    }
}
