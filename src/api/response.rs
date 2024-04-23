use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    status: u16,
    message: String,
    data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(status: u16, message: String, data: T) -> ApiResponse<T> {
        ApiResponse { status, message, data }
    }
}
