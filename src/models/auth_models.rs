use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Claims {
    email: String,
    account_type: String,
    exp: usize,
}
