use sea_query::enum_def;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[enum_def]
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct Item {
    pub id: i32,
    pub access_token: String,
    pub credentials_id: i32,
    pub institution_id: i32,
    pub webhook: String,
}
