use std::collections::HashMap;

use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use super::sql::{select_all_from_transaction, select_trx_by_id};

#[utoipa::path(
    get,
    path = "/transactions/{id}",
    responses(
        (status = OK, body = Transaction),
        (status = NOT_FOUND)
    ),
    params(
        ("id" = i32, Path, description = "identifier for a single transaction"),
    ),
    tag = "Transactions"
)]
pub async fn get_transaction_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    Json(select_trx_by_id(id, &app_state.db).await)
}

#[utoipa::path(
    get,
    path = "/transactions",
    responses(
        (status = OK, body = [Transaction]),
        (status = NOT_FOUND)
    ),
    params(
        ("id" = i32, Path, description = "identifier for a single transaction"),
    ),
    tag = "Transactions"
)]
pub async fn get_all_transactions(
    Query(params): Query<HashMap<String, i32>>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    Json(select_all_from_transaction(*params.get("account_id").unwrap(), &app_state.db).await)
}

pub fn api() -> Router<AppState> {
    Router::new()
        .route("/transactions/:id", get(get_transaction_by_id))
        .route("/transactions", get(get_all_transactions))
}
