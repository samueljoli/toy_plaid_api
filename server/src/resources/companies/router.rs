use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::AppState;

use super::sql::{select_account_by_id, select_account_by_slug};

#[utoipa::path(
    get,
    path = "/companies/{id}",
    responses(
        (status = OK, body = Company),
        (status = NOT_FOUND)
    ),
    params(
        ("id" = i32, Path, description = "identifier for a single transaction"),
    )
)]
pub async fn get_company_by_id(
    Path(id): Path<i32>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    Json(select_account_by_id(id, &app_state.db).await)
}

#[utoipa::path(
    get,
    path = "/companies/{slug}",
    responses(
        (status = OK, body = Company),
        (status = NOT_FOUND)
    ),
    params(
        ("slug" = String, Path, description = "slug for a single company"),
    )
)]
pub async fn get_company_by_slug(
    Path(slug): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    Json(select_account_by_slug(&slug, &app_state.db).await)
}

pub fn api() -> Router<AppState> {
    Router::new()
        .route("/companies/:id", get(get_company_by_id))
        .route("/companies/:slug", get(get_company_by_slug))
}
