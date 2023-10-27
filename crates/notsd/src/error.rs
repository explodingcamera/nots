use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use hyper::Body;
use serde_json::json;

pub struct Error(pub String, pub u16);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.1).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let error_json = json!({ "error": self.0 }).to_string();
        let response = Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(Body::from(error_json))
            .unwrap();

        response.into_response()
    }
}
