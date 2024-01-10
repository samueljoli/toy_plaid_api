use sea_query::enum_def;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[enum_def]
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct Credential {
    pub id: i32,
    pub email: String,
    pub password: String,
}
