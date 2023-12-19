use super::{models::Transactions, sql::select_trx_by_id};
use crate::AppState;
use axum::{response::IntoResponse, routing::get, Json, Router};

#[utoipa::path(
    get,
    path = "/transactions/{id}",
    responses(
        (status = 200),
        (status = NOT_FOUND)
    ),
    params(
        ("id" = u64, Path, description = "identifier for a single transaction"),
    )
)]
pub async fn get_transaction_by_id() -> impl IntoResponse {
    select_trx_by_id();

    Json(Transactions {
        name: String::from("Plaid API"),
    })
}

pub fn api() -> Router<AppState> {
    Router::new().route("/transactions/:id", get(get_transaction_by_id))
}
