use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[derive(Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}
