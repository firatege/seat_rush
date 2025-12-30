use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

// ---- RESERVE ----

#[derive(Deserialize)]
pub struct ReserveQuery {
    pub seat_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ReserveResponse {
    pub seat_id: i32,
}

// ---- BUY ----

#[derive(Deserialize, Validate)]
pub struct BuyRequest {
    #[validate(range(min = 1))]
    pub cost: i32,
}

#[derive(Deserialize, Validate)]
pub struct BuyRequestPath {
    #[validate(range(min = 1))]
    pub seat_id: i32,
}
#[derive(Serialize)]
pub struct BuyResponse {
    pub ticket_id: Uuid,
    pub seat_id: i32,
    pub cost: i32,
    pub user_id: Uuid,
}

// ---- CANCEL ----

#[derive(Deserialize, Validate)]
pub struct CancelRequest {
    #[validate(range(min = 1))]
    pub seat_id: i32,
}



#[derive(Serialize)]
pub struct CancelResponse {
    pub seat_id: i32,
    pub canceled_by: Uuid,
}
