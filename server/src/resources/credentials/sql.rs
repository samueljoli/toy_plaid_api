use sea_query::{PostgresQueryBuilder, Query};
use sqlx::Postgres;

use super::models::{Credentials, CredentialsIden};

pub async fn insert_credential(
    email: String,
    password: String,
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Credentials {
    // TODO: Refactor to get or insert
    let _thing = &mut **trx;
    let query = Query::insert()
        .into_table(CredentialsIden::Table)
        .columns(vec![CredentialsIden::Email, CredentialsIden::Password])
        .values_panic(vec![email.into(), password.into()])
        .returning_all()
        .to_string(PostgresQueryBuilder)
        .to_owned();

    sqlx::query_as::<Postgres, Credentials>(&query)
        .fetch_one(&mut **trx)
        .await
        .unwrap()
}
