use sea_query::{PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};

use super::models::{Credentials, CredentialsIden};

pub async fn insert_credential(
    email: String,
    password: String,
    db: &Pool<Postgres>,
) -> Credentials {
    // TODO: Refactor to get or insert
    let query = Query::insert()
        .into_table(CredentialsIden::Table)
        .columns(vec![CredentialsIden::Email, CredentialsIden::Password])
        .values_panic(vec![email.into(), password.into()])
        .returning_all()
        .to_string(PostgresQueryBuilder)
        .to_owned();

    sqlx::query_as::<Postgres, Credentials>(&query)
        .fetch_one(db)
        .await
        .unwrap()
}
