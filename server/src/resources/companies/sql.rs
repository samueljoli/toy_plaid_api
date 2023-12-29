use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use super::models::{Company, CompanyIden};

/// SELECT "company"."id", "company"."name" FROM "company" WHERE "company"."id" = $1
pub async fn select_account_by_id(id: i32, db: &Pool<Postgres>) -> Company {
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

    sqlx::query::<Postgres>(&query)
        .map(|row: PgRow| Company {
            id: row.try_get("id").unwrap(),
            name: row.try_get("name").unwrap(),
            slug: row.try_get("slug").unwrap(),
        })
        .fetch_one(db)
        .await
        .unwrap()
}

/// SELECT "company"."id", "company"."name" FROM "company" WHERE "company"."slug" = $1
pub async fn select_account_by_slug(slug: &str, db: &Pool<Postgres>) -> Company {
    let query = Query::select()
        .from(CompanyIden::Table)
        .columns(vec![
            CompanyIden::Table,
            CompanyIden::Id,
            CompanyIden::Name,
            CompanyIden::Slug,
        ])
        .and_where(Expr::col((CompanyIden::Table, CompanyIden::Slug)).eq(slug))
        .to_string(PostgresQueryBuilder);

    sqlx::query::<Postgres>(&query)
        .map(|row: PgRow| Company {
            id: row.try_get("id").unwrap(),
            name: row.try_get("name").unwrap(),
            slug: row.try_get("slug").unwrap(),
        })
        .fetch_one(db)
        .await
        .unwrap()
}
