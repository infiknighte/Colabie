use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bitcode::{DecodeOwned, Encode};

pub struct BitCode<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for BitCode<T>
where
    T: DecodeOwned,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        Ok(Self(
            bitcode::decode(&bytes).map_err(|_| StatusCode::BAD_REQUEST)?,
        ))
    }
}

impl<T> IntoResponse for BitCode<T>
where
    T: Encode,
{
    fn into_response(self) -> Response {
        bitcode::encode(&self.0).into_response()
    }
}
