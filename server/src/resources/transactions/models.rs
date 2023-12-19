use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Transactions {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct ApiResponse {
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
    item: Item,
    total_transactions: u32,
    request_id: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct Account {
    account_id: String,
    // balances: Balances,
    mask: String,
    name: String,
    official_name: String,
    subtype: String,
    r#type: String,
}

// #[derive(Serialize, Deserialize, Debug, ToSchema)]
// struct Balances {
//     available: Option<f64>,
//     current: f64,
//     iso_currency_code: String,
//     limit: Option<f64>,
//     unofficial_currency_code: Option<String>,
// }

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct Transaction {
    account_id: String, // Foreign key Account
    amount: f64,
    iso_currency_code: String,
    unofficial_currency_code: Option<String>,
    check_number: Option<String>,
    // counterparties: Vec<Counterparty>, // JOIN table
    date: String,
    datetime: String,
    authorized_date: String,
    authorized_datetime: String,
    location: Location, // Foreign key to Location
    name: String,
    merchant_name: String,
    merchant_entity_id: String,
    logo_url: String,
    website: String,
    payment_meta: PaymentMeta, // Foreign key
    payment_channel: String,
    pending: bool,
    pending_transaction_id: Option<String>,
    personal_finance_category: PersonalFinanceCategory, // Foreign key
    personal_finance_category_icon_url: String,
    transaction_id: String,
    transaction_code: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug, ToSchema)]
// struct Counterparty {
//     name: String,
//     r#type: String,
//     logo_url: String,
//     website: String,
//     entity_id: String,
//     confidence_level: String,
// }

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct Location {
    address: Option<String>,
    city: Option<String>,
    region: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
    store_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct PaymentMeta {
    by_order_of: Option<String>,
    payee: Option<String>,
    payer: Option<String>,
    payment_method: Option<String>,
    payment_processor: Option<String>,
    ppd_id: Option<String>,
    reason: Option<String>,
    reference_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct PersonalFinanceCategory {
    primary: String,
    detailed: String,
    confidence_level: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct Item {
    available_products: Vec<String>,
    billed_products: Vec<String>,
    consent_expiration_time: Option<String>,
    error: Option<String>,
    institution_id: String,
    item_id: String,
    update_type: String,
    webhook: String,
}

// Join tables
#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct TransactionCounterparty {
    transaction_id: String,
    counterparty_id: String,
}
