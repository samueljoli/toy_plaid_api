use std::collections::HashMap;

use crate::{
    resources::{
        accounts::sql::insert_account,
        credentials::sql::get_or_create_credential,
        institutions::sql::select_institution_by_name,
        personal_finance_categories::sql::select_all_categories,
        transactions::models::{Transaction, TransactionIden},
    },
    AppState,
};
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use sea_query::{PostgresQueryBuilder, Query};
use serde::Deserialize;

use sqlx::Postgres;

use super::{
    models::{Item, ItemIden},
    sql::{insert_item, select_item_by_credential_id},
    tasks::fire_webhook,
    utils::get_transactions_from_csv,
};

#[derive(Deserialize)]
pub struct CreateItem {
    pub email: String,
    pub password: String,
    pub webhook: String,
}

pub async fn categories_name_to_id_map(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();

    let categories = select_all_categories(trx).await;

    for category in categories {
        map.insert(category.detailed, category.id);
    }

    map
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
    let mut trx = app_state.db.begin().await.unwrap();

    let (credential, was_created) =
        get_or_create_credential(payload.email.clone(), payload.password.clone(), &mut trx).await;

    if !was_created {
        let item = select_item_by_credential_id(credential.id, &mut trx).await;

        Json(item)
    } else {
        let institution = select_institution_by_name("Brex", &mut trx).await;

        let item = insert_item(credential, institution.id, payload.webhook, &mut trx).await;

        let account = insert_account(item.id, &mut trx).await;

        let map = categories_name_to_id_map(&mut trx).await;

        let transactions = get_transactions_from_csv();

        let mut builder = Query::insert()
            .into_table(TransactionIden::Table)
            .columns(vec![
                TransactionIden::AccountId,
                TransactionIden::Amount,
                TransactionIden::IsoCurrencyCode,
                TransactionIden::Date,
                TransactionIden::Datetime,
                TransactionIden::Name,
                TransactionIden::MerchantName,
                TransactionIden::PaymentChannel,
                TransactionIden::Pending,
                TransactionIden::PersonalFinanceCategoryId,
            ])
            .to_owned();

        for transaction in transactions {
            let category_id = map
                .get(&transaction.personal_finance_category.to_string())
                .unwrap();

            let category_id_value = (*category_id).into();

            builder.values_panic(vec![
                account.id.into(),
                transaction.amount.into(),
                "USD".into(),
                transaction.date.into(),
                transaction.datetime.into(),
                transaction.name.into(),
                transaction.merchant_name.into(),
                transaction.payment_channel.into(),
                false.into(),
                category_id_value,
            ]);
        }

        let query = builder.to_string(PostgresQueryBuilder);

        sqlx::query_as::<Postgres, Transaction>(&query)
            .fetch_all(&mut *trx)
            .await
            .unwrap();

        trx.commit().await.unwrap();

        app_state
            .celery_app
            .send_task(fire_webhook::new(item.id, item.webhook.clone()).with_countdown(5))
            .await
            .unwrap();

        Json(item)
    }
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
    let query = Query::select()
        .from(ItemIden::Table)
        .columns(vec![
            ItemIden::Id,
            ItemIden::AccessToken,
            ItemIden::InstitutionId,
            ItemIden::CredentialId,
            ItemIden::Webhook,
        ])
        .to_string(PostgresQueryBuilder);

    let items = sqlx::query_as::<Postgres, Item>(&query)
        .fetch_all(&app_state.db)
        .await
        .unwrap();

    Json(items)
}

pub fn api() -> Router<AppState> {
    Router::new()
        .route("/item", post(post_item))
        .route("/webhook", post(webhook))
}
