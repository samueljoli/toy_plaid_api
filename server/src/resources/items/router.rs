use crate::{
    resources::{
        credentials::sql::insert_credential, institutions::sql::select_institution_by_name,
    },
    AppState,
};
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use serde_derive::Deserialize;

use super::{sql::insert_item, tasks::add};

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
    State(app_state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> impl IntoResponse {
    let credential = insert_credential(payload.email, payload.password, &app_state.db).await;

    let institution = select_institution_by_name("Brex", &app_state.db).await;

    let item = insert_item(credential, institution.id, payload.webhook, &app_state.db).await;

    Json(item)
}

#[utoipa::path(
    post,
    path = "/webhook",
    responses(
        (status = OK),
    ),
    tag = "Items",
)]
pub async fn webhook(State(app_state): State<AppState>) -> impl IntoResponse {
    app_state
        .celery_app
        .send_task(add::new(1, 2).with_countdown(5))
        .await
        .unwrap();
}

pub fn api() -> Router<AppState> {
    Router::new()
        .route("/item", post(post_item))
        .route("/webhook", post(webhook))
}
