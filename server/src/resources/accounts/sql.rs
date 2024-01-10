use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};

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

    sqlx::query_as::<Postgres, Account>(&query)
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

    sqlx::query_as::<Postgres, Account>(&query)
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn insert_account(
    item_id: i32,
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Account {
    let query = Query::insert()
        .into_table(AccountIden::Table)
        .columns(vec![
            AccountIden::Mask,
            AccountIden::Name,
            AccountIden::OfficialName,
            AccountIden::Subtype,
            AccountIden::AccountType,
            AccountIden::ItemId,
        ])
        .values_panic(vec![
            "0000".into(),
            "Plaid Checking".into(),
            "Plaid Gold Checking".into(),
            "checking".into(),
            "depository".into(),
            item_id.into(),
        ])
        .returning_all()
        .to_string(PostgresQueryBuilder)
        .to_owned();

    sqlx::query_as::<Postgres, Account>(&query)
        .fetch_one(&mut **trx)
        .await
        .unwrap()
}
