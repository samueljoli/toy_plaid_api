use base64::{engine::general_purpose, Engine as _};
use sea_query::{PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};

use crate::resources::credentials::models::Credentials;

use super::models::{Item, ItemIden};

pub async fn insert_item(
    credentials: Credentials,
    institution_id: i32,
    webhook: String,
    db: &Pool<Postgres>,
) -> Item {
    let access_token: String = general_purpose::STANDARD_NO_PAD
        .encode(format!("{}-{}", credentials.email, credentials.password));

    let query = Query::insert()
        .into_table(ItemIden::Table)
        .columns(vec![
            ItemIden::AccessToken,
            ItemIden::CredentialsId,
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
        .fetch_one(db)
        .await
        .unwrap()
}
