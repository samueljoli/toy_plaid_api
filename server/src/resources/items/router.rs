use crate::{
    resources::{
        credentials::sql::insert_credential, institutions::sql::select_institution_by_name,
    },
    AppState,
};
use axum::{extract::State, response::IntoResponse, Json};
use serde_derive::Deserialize;

use super::sql::insert_item;

#[derive(Deserialize)]
pub struct CreateItem {
    pub email: String,
    pub password: String,
    pub webhook: String,
}

#[utoipa::path(
    post,
    path = "/item",
    responses(
        (status = OK),
    ),
    tag = "Items",
)]
pub async fn post_item(
    Json(payload): Json<CreateItem>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let credential = insert_credential(payload.email, payload.password, &app_state.db).await;

    let institution = select_institution_by_name("Brex", &app_state.db).await;

    let item = insert_item(credential, institution.id, payload.webhook, &app_state.db).await;

    // create account
    // create map of name to id map
    // create transactions

    Json(item)
}

#[utoipa::path(
    post,
    path = "/item",
    responses(
        (status = OK),
    ),
    tag = "Items",
)]
pub async fn webhook() -> impl IntoResponse {
    // Fallback
}
