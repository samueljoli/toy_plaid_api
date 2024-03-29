use base64::{engine::general_purpose, Engine as _};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::Postgres;

use crate::resources::credentials::models::Credential;

use super::models::{Item, ItemIden};

pub async fn insert_item(
    credentials: Credential,
    institution_id: i32,
    webhook: String,
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Item {
    let access_token: String = general_purpose::STANDARD_NO_PAD
        .encode(format!("{}-{}", credentials.email, credentials.password));

    let query = Query::insert()
        .into_table(ItemIden::Table)
        .columns(vec![
            ItemIden::AccessToken,
            ItemIden::CredentialId,
            ItemIden::InstitutionId,
            ItemIden::Webhook,
        ])
        .values_panic(vec![
            access_token.into(),
            credentials.id.into(),
            institution_id.into(),
            webhook.into(),
        ])
        .returning_all()
        .to_string(PostgresQueryBuilder)
        .to_owned();

    sqlx::query_as::<Postgres, Item>(&query)
        .fetch_one(&mut **trx)
        .await
        .unwrap()
}

pub async fn select_item_by_id(id: i32, trx: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> Item {
    let query = Query::select()
        .from(ItemIden::Table)
        .columns(vec![
            ItemIden::Id,
            ItemIden::AccessToken,
            ItemIden::CredentialId,
            ItemIden::InstitutionId,
            ItemIden::Webhook,
        ])
        .and_where(Expr::col(ItemIden::Id).eq(id))
        .to_string(PostgresQueryBuilder);

    sqlx::query_as::<Postgres, Item>(&query)
        .fetch_one(&mut **trx)
        .await
        .unwrap()
}

pub async fn select_item_by_credential_id(
    credential_id: i32,
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Item {
    let query = Query::select()
        .from(ItemIden::Table)
        .columns(vec![
            ItemIden::Id,
            ItemIden::AccessToken,
            ItemIden::CredentialId,
            ItemIden::InstitutionId,
            ItemIden::Webhook,
        ])
        .and_where(Expr::col(ItemIden::CredentialId).eq(credential_id))
        .to_string(PostgresQueryBuilder);

    sqlx::query_as::<Postgres, Item>(&query)
        .fetch_one(&mut **trx)
        .await
        .unwrap()
}
