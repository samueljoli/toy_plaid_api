use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[enum_def] // => Generates TransactionIden
#[derive(FromRow, Serialize, Deserialize, Debug, ToSchema)]
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

#[enum_def] // => Generates WebhookIden
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Webhook {
    pub webhook_type: String,
    pub webhook_code: String,
    pub item_id: String,
    pub initial_update_complete: bool,
    pub historical_update_complete: bool,
    pub environment: String,
}

#[enum_def] // => Generates AuthIden
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Item {
    pub id: String,
    pub access_token: String,
    pub webhook_url: String,
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
