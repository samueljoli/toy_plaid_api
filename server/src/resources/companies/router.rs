use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::AppState;

use super::sql::{select_company_by_id, select_transactions_by_account};

#[utoipa::path(
    get,
    path = "/companies/{id}",
    responses(
        (status = OK, body = Company),
        (status = NOT_FOUND)
    ),
    params(
        ("id" = i32, Path, description = "identifier for a single transaction"),
    ),
    tag = "Companies"
)]
pub async fn get_company_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    Json(select_company_by_id(id, &app_state.db).await)
}

pub async fn transactions_dashboard(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    Json(select_transactions_by_account(id, &app_state.db).await)
}

pub fn api() -> Router<AppState> {
    Router::new()
        .route("/companies/:id", get(get_company_by_id))
        .route("/companies/:id/dashboard", get(transactions_dashboard))
}
