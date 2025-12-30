use crate::utils::error::GatewayError;

pub type GatewayResult<T> = Result<T, GatewayError>;