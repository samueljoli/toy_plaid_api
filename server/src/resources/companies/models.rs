use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[enum_def] // => Generates CompanyIden
#[derive(FromRow, Serialize, Deserialize, Debug, ToSchema)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub slug: String,
}
