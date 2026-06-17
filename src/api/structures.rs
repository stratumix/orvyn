use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub(crate) message: String,
}

impl Default for ErrorResponse {
    fn default() -> Self {
        Self {
            message: "Unexpected behaviour has occured".to_string(),
        }
    }
}
