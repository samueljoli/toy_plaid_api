use crate::AppState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use super::sql::{select_account_by_id, select_all_from_account};

#[utoipa::path(
    get,
    path = "/accounts/{id}",
    responses(
        (status = OK, body = Account),
        (status = NOT_FOUND)
    ),
    params(
        ("id" = i32, Path, description = "identifier for a single account"),
    )
)]
pub async fn get_account_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    Json(select_account_by_id(id, &app_state.db).await)
}

#[utoipa::path(
    get,
    path = "/accounts",
    responses(
        (status = OK, body = Transaction),
        (status = NOT_FOUND)
    ),
    params(
        ("id" = i32, Path, description = "identifier for a single transaction"),
    )
)]
pub async fn get_all_accounts(State(app_state): State<AppState>) -> impl IntoResponse {
    Json(select_all_from_account(&app_state.db).await)
}

pub fn api() -> Router<AppState> {
    Router::new()
        .route("/accounts/:id", get(get_account_by_id))
        .route("/accounts", get(get_all_accounts))
}
