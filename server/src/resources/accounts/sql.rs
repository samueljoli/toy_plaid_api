use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use super::models::{Account, AccountIden};

pub async fn select_account_by_id(id: i32, db: &Pool<Postgres>) -> Account {
    let query = Query::select()
        .from(AccountIden::Table)
        .columns(vec![
            AccountIden::Id,
            AccountIden::Mask,
            AccountIden::Name,
            AccountIden::OfficialName,
            AccountIden::Subtype,
        ])
        .and_where(Expr::col(AccountIden::Id).eq(id))
        .to_string(PostgresQueryBuilder);

    sqlx::query::<Postgres>(&query)
        .map(|row: PgRow| Account {
            id: row.try_get(0).unwrap(),
            mask: row.try_get("mask").unwrap(),
            name: row.try_get("name").unwrap(),
            official_name: row.try_get("official_name").unwrap(),
            r#type: row.try_get("type").unwrap(),
            subtype: row.try_get("subtype").unwrap(),
        })
        .fetch_one(db)
        .await
        .unwrap()
}

pub async fn select_all_from_account(db: &Pool<Postgres>) -> Vec<Account> {
    let query = Query::select()
        .from(AccountIden::Table)
        .columns(vec![
            AccountIden::Id,
            AccountIden::Mask,
            AccountIden::Name,
            AccountIden::OfficialName,
            AccountIden::Subtype,
        ])
        .to_string(PostgresQueryBuilder);

    sqlx::query::<Postgres>(&query)
        .map(|row: PgRow| Account {
            id: row.try_get(0).unwrap(),
            mask: row.try_get("mask").unwrap(),
            name: row.try_get("name").unwrap(),
            official_name: row.try_get("official_name").unwrap(),
            r#type: row.try_get("type").unwrap(),
            subtype: row.try_get("subtype").unwrap(),
        })
        .fetch_all(db)
        .await
        .unwrap()
}
