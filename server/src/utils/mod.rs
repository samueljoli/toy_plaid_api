use axum::response::IntoResponse;

pub fn get_config_directory() -> String {
    let mut path = std::env::current_dir().expect("Failed to get current directory");

    // If executed from target (e.g., tests), move up one directory
    if path.ends_with("server") {
        path.pop();
    }

    path.push("config");

    path.to_str()
        .expect("Failed to convert path to string")
        .to_string()
}

/// Meant to be used as a fallback if the candidate is unable
/// to complete the BE exercise. This endpoint returns a
/// reasonable contract that should be able to support the
/// transactions dashboard.
/// contract:
/// {
///     company_id: 1,
///     company_name: "Standard Metrics",
///     accounts: [
///         {
///             account_id: 1,
///             transactions: [
///               {
///                   "id": 1,
///                   "amount": 10000.0,
///                   "iso_currency_code": "USD",
///                   "date": "2023-11-01",
///                   "datetime": "2023-11-01 02:37:32",
///                   "name": "Initial Deposit",
///                   "merchant_name": "ATM Deposit",
///                   "payment_channel": "online",
///                   "pending": false,
///                   "category": "TRANSFER_IN_DEPOSIT"
///               }
///
///             ]
///         }
///     ],
/// }
pub async fn fallback_demo_endpoint() -> impl IntoResponse {
    // let sql = Query::select()
    //     .columns(vec![
    //         AccountIden::Table,
    //         AccountIden::
    //     ])
}
