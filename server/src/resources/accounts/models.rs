use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[enum_def] // => Generates AccountIden
#[derive(FromRow, Serialize, Deserialize, Debug, ToSchema)]
pub struct Account {
    pub id: i32,
    pub mask: String,
    pub name: String,
    pub official_name: String,
    pub r#type: String,
    pub subtype: String,
    pub item_id: i32,
}
