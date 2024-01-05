use std::collections::HashMap;

use sea_query::{Alias, Expr, Order, PostgresQueryBuilder, Query};
use serde_derive::{Deserialize, Serialize};
use sqlx::{
    postgres::PgRow,
    types::chrono::{NaiveDate, NaiveDateTime},
    FromRow, Pool, Postgres, Row,
};

use crate::resources::{
    accounts::models::AccountIden,
    items::models::ItemIden,
    transactions::models::{PersonalFinanceCategoryIden, TransactionIden, TransactionSQL},
};

use super::models::{Company, CompanyIden};

/// SELECT "company"."id", "company"."name" FROM "company" WHERE "company"."id" = $1
pub async fn select_company_by_id(id: i32, db: &Pool<Postgres>) -> Company {
    let query = Query::select()
        .from(CompanyIden::Table)
        .columns(vec![
            CompanyIden::Table,
            CompanyIden::Id,
            CompanyIden::Name,
            CompanyIden::Slug,
        ])
        .and_where(Expr::col((CompanyIden::Table, CompanyIden::Id)).eq(id))
        .to_string(PostgresQueryBuilder);

    sqlx::query_as::<Postgres, Company>(&query)
        .fetch_one(db)
        .await
        .unwrap()
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct AccountWithTransactions {
    pub account_id: i32,
    pub transactions: Vec<TransactionSQL>,
}
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CompanyWithAccounts {
    pub company_id: i32,
    pub company_name: String,
    pub accounts: Vec<AccountWithTransactions>,
}
/// SELECT "company"."id",
///        "company"."name",
///        "company"."slug",
///        "account"."id",
///        "transaction"."id",
///        "transaction"."account_id",
///        "transaction"."amount",
///        "transaction"."iso_currency_code",
///        "transaction"."date",
///        "transaction"."datetime",
///        "transaction"."name",
///        "transaction"."merchant_name",
///        "transaction"."payment_channel",
///        "transaction"."pending",
///        "transaction"."personal_finance_category_id",
///        "personal_finance_category"."detailed",
///        "company"."id"   AS "company_id",
///        "company"."name" AS "company_name",
///        "account"."id"   AS "account_id",
///        "detailed"       AS "category"
/// FROM   "company"
///        LEFT JOIN "account"
///               ON "company"."id" = "account"."company_id"
///        LEFT JOIN "transaction"
///               ON "account"."id" = "transaction"."account_id"
///        LEFT JOIN "personal_finance_category"
///               ON "transaction"."personal_finance_category_id" =
///                  "personal_finance_category"."id"
/// WHERE  "company"."id" = 1
/// ORDER  BY "transaction"."datetime" DESC
pub async fn select_transactions_by_account(
    item_id: i32,
    db: &Pool<Postgres>,
) -> CompanyWithAccounts {
    let query = Query::select()
        .columns(vec![(ItemIden::Table, ItemIden::Id)])
        .column((AccountIden::Table, AccountIden::Id))
        .columns(vec![
            (TransactionIden::Table, TransactionIden::Id),
            (TransactionIden::Table, TransactionIden::AccountId),
            (TransactionIden::Table, TransactionIden::Amount),
            (TransactionIden::Table, TransactionIden::IsoCurrencyCode),
            (TransactionIden::Table, TransactionIden::Date),
            (TransactionIden::Table, TransactionIden::Datetime),
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
        .from(ItemIden::Table)
        .left_join(
            AccountIden::Table,
            Expr::col((ItemIden::Table, ItemIden::Id))
                .equals((AccountIden::Table, AccountIden::ItemId)),
        )
        .left_join(
            TransactionIden::Table,
            Expr::col((AccountIden::Table, AccountIden::Id))
                .equals((TransactionIden::Table, TransactionIden::AccountId)),
        )
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
        .expr_as(
            Expr::col((ItemIden::Table, ItemIden::Id)),
            Alias::new("item_id"),
        )
        .expr_as(
            Expr::col((AccountIden::Table, AccountIden::Id)),
            Alias::new("account_id"),
        )
        .expr_as(
            Expr::col(PersonalFinanceCategoryIden::Detailed),
            Alias::new("category"),
        )
        .and_where(Expr::col((ItemIden::Table, ItemIden::Id)).eq(item_id))
        .order_by(
            (TransactionIden::Table, TransactionIden::Datetime),
            Order::Desc,
        )
        .to_string(PostgresQueryBuilder);

    let results = sqlx::query::<Postgres>(&query)
        .map(|row: PgRow| TransactionSQL {
            id: row.try_get("id").unwrap(),
            account_id: row.try_get("account_id").unwrap(),
            amount: row.try_get("amount").unwrap(),
            name: row.try_get("name").unwrap(),
            iso_currency_code: row.try_get("iso_currency_code").unwrap(),
            date: row
                .try_get::<NaiveDate, _>("date")
                .unwrap()
                .format("%Y-%m-%d")
                .to_string(),
            datetime: row
                .try_get::<NaiveDateTime, _>("datetime")
                .unwrap()
                .to_string(),
            merchant_name: row.try_get("merchant_name").unwrap(),
            category: row.try_get("category").unwrap(),
            payment_channel: row.try_get("payment_channel").unwrap(),
            pending: row.try_get("pending").unwrap(),
        })
        .fetch_all(db)
        .await
        .unwrap();

    let company = select_company_by_id(item_id, db).await;

    let mut accounts: HashMap<i32, Vec<TransactionSQL>> = HashMap::new();

    for result in results {
        let transaction = TransactionSQL {
            id: result.id,
            account_id: result.account_id,
            amount: result.amount,
            iso_currency_code: result.iso_currency_code,
            date: result.date,
            datetime: result.datetime,
            name: result.name,
            merchant_name: result.merchant_name,
            payment_channel: result.payment_channel,
            pending: result.pending,
            category: result.category,
        };

        accounts
            .entry(result.account_id)
            .or_insert_with(Vec::new)
            .push(transaction);
    }

    let accounts_with_transactions = accounts
        .into_iter()
        .map(|(account_id, transactions)| AccountWithTransactions {
            account_id,
            transactions,
        })
        .collect();

    CompanyWithAccounts {
        company_id: item_id,
        company_name: company.name,
        accounts: accounts_with_transactions,
    }
}
