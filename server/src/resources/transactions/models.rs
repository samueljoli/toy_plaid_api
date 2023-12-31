use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[enum_def] // => Generates TransactionIden
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Transaction {
    pub id: i32,
    pub account_id: i32, // Foreign key Account
    pub amount: f64,
    pub iso_currency_code: String,
    pub date: String,
    pub datetime: String,
    pub name: String,
    pub merchant_name: String,
    pub payment_channel: String,
    pub pending: bool,
    pub personal_finance_category_id: i32, // Foreign key PersonalFinanceCategory
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct TransactionSQL {
    pub id: i32,
    pub account_id: i32, // Foreign key Account
    pub amount: f64,
    pub category: String,
    pub date: String,
    pub datetime: String,
    pub iso_currency_code: String,
    pub merchant_name: String,
    pub name: String,
    pub payment_channel: String,
    pub pending: bool,
}

#[enum_def] // => Generates PersonalFinanceCategoryIden NOTE: Move
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PersonalFinanceCategory {
    pub id: i32,
    pub primary: String,
    pub detailed: String,
}
