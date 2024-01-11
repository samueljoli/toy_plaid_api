use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::Postgres;

use super::models::{Credential, CredentialIden};

pub async fn insert_credential(
    email: String,
    password: String,
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<Credential, sqlx::Error> {
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
}

pub async fn select_credential_by_email_and_password(
    email: String,
    password: String,
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<Credential, sqlx::Error> {
    let query = Query::select()
        .from(CredentialIden::Table)
        .columns(vec![
            CredentialIden::Id,
            CredentialIden::Email,
            CredentialIden::Password,
        ])
        .and_where(Expr::col(CredentialIden::Email).eq(email))
        .and_where(Expr::col(CredentialIden::Password).eq(password))
        .to_string(PostgresQueryBuilder);

    sqlx::query_as::<Postgres, Credential>(&query)
        .fetch_one(&mut **trx)
        .await
}

pub async fn get_or_create_credential(
    email: String,
    password: String,
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> (Credential, bool) {
    let credential =
        select_credential_by_email_and_password(email.clone(), password.clone(), trx).await;

    match credential {
        Ok(credential) => (credential, false),
        Err(_) => {
            let credential = insert_credential(email, password, trx).await.unwrap();
            (credential, true)
        }
    }
}
