use sea_query::enum_def;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;

#[enum_def]
#[derive(Deserialize, Serialize, FromRow)]
pub struct PersonalFinanceCategory {
    pub id: i32,
    pub detailed: String,
    pub primary_category: String,
}
