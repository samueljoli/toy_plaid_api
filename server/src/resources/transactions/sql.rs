use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{postgres::PgRow, types::chrono::NaiveDate, Pool, Postgres, Row};

use crate::resources::transactions::models::PersonalFinanceCategoryIden;

use super::models::{Transaction, TransactionIden, TransactionSQL};

pub async fn select_trx_by_id(id: i32, db: &Pool<Postgres>) -> Transaction {
    let query = Query::select()
        .from(TransactionIden::Table)
        .columns(vec![
            TransactionIden::Id,
            TransactionIden::AccountId,
            TransactionIden::Amount,
            TransactionIden::IsoCurrencyCode,
            TransactionIden::Date,
            TransactionIden::Name,
            TransactionIden::MerchantName,
            TransactionIden::PaymentChannel,
            TransactionIden::Pending,
            TransactionIden::PersonalFinanceCategoryId,
        ])
        .and_where(Expr::col(TransactionIden::Id).eq(id))
        .to_string(PostgresQueryBuilder);

    sqlx::query::<Postgres>(&query)
        .map(|row: PgRow| Transaction {
            id: row.try_get("id").unwrap(),
            account_id: row.try_get("account_id").unwrap(),
            amount: row.try_get("amount").unwrap(),
            iso_currency_code: row.try_get("iso_currency_code").unwrap(),
            date: row
                .try_get::<NaiveDate, _>("date")
                .unwrap()
                .format("%Y-%m-%d")
                .to_string(),
            name: row.try_get("name").unwrap(),
            merchant_name: row.try_get("merchant_name").unwrap(),
            payment_channel: row.try_get("payment_channel").unwrap(),
            pending: row.try_get("pending").unwrap(),
            personal_finance_category_id: row.try_get("personal_finance_category_id").unwrap(),
        })
        .fetch_one(db)
        .await
        .unwrap()
}

pub async fn select_all_from_transaction(
    account_id: i32,
    db: &Pool<Postgres>,
) -> Vec<TransactionSQL> {
    let query = Query::select()
        .columns(vec![
            (TransactionIden::Table, TransactionIden::Id),
            (TransactionIden::Table, TransactionIden::AccountId),
            (TransactionIden::Table, TransactionIden::Amount),
            (TransactionIden::Table, TransactionIden::IsoCurrencyCode),
            (TransactionIden::Table, TransactionIden::Date),
            (TransactionIden::Table, TransactionIden::Name),
            (TransactionIden::Table, TransactionIden::MerchantName),
            (TransactionIden::Table, TransactionIden::PaymentChannel),
            (TransactionIden::Table, TransactionIden::Pending),
            (
                TransactionIden::Table,
                TransactionIden::PersonalFinanceCategoryId,
            ),
        ])
        .column((
            PersonalFinanceCategoryIden::Table,
            PersonalFinanceCategoryIden::Detailed,
        ))
        .from(TransactionIden::Table)
        .and_where(Expr::col((TransactionIden::Table, TransactionIden::AccountId)).eq(account_id))
        .left_join(
            PersonalFinanceCategoryIden::Table,
            Expr::col((
                TransactionIden::Table,
                TransactionIden::PersonalFinanceCategoryId,
            ))
            .equals((
                PersonalFinanceCategoryIden::Table,
                PersonalFinanceCategoryIden::Id,
            )),
        )
        .to_string(PostgresQueryBuilder);

    sqlx::query::<Postgres>(&query)
        .map(|row: PgRow| TransactionSQL {
            id: row.try_get("id").unwrap(),
            account_id: row.try_get("account_id").unwrap(),
            amount: row.try_get("amount").unwrap(),
            iso_currency_code: row.try_get("iso_currency_code").unwrap(),
            date: row
                .try_get::<NaiveDate, _>("date")
                .unwrap()
                .format("%Y-%m-%d")
                .to_string(),
            name: row.try_get("name").unwrap(),
            merchant_name: row.try_get("merchant_name").unwrap(),
            payment_channel: row.try_get("payment_channel").unwrap(),
            pending: row.try_get("pending").unwrap(),
            category: row.try_get("detailed").unwrap(),
        })
        .fetch_all(db)
        .await
        .unwrap()
}
