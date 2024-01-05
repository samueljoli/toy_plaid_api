use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[enum_def]
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct Institution {
    pub id: i32,
    pub name: String,
}
