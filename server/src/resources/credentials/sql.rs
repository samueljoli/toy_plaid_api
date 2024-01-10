use sea_query::{PostgresQueryBuilder, Query};
use sqlx::Postgres;

use super::models::{Credential, CredentialIden};

pub async fn insert_credential(
    email: String,
    password: String,
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Credential {
    // TODO: Refactor to get or insert
    let _thing = &mut **trx;
    let query = Query::insert()
        .into_table(CredentialIden::Table)
        .columns(vec![CredentialIden::Email, CredentialIden::Password])
        .values_panic(vec![email.into(), password.into()])
        .returning_all()
        .to_string(PostgresQueryBuilder)
        .to_owned();

    sqlx::query_as::<Postgres, Credential>(&query)
        .fetch_one(&mut **trx)
        .await
        .unwrap()
}
