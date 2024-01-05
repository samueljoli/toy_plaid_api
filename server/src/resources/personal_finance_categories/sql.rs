use sea_query::{PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};

use super::models::{PersonalFinanceCategory, PersonalFinanceCategoryIden};

pub async fn select_all_categories(db: &Pool<Postgres>) -> Vec<PersonalFinanceCategory> {
    let query = Query::select()
        .from(PersonalFinanceCategoryIden::Table)
        .columns(vec![
            PersonalFinanceCategoryIden::Id,
            PersonalFinanceCategoryIden::Detailed,
            PersonalFinanceCategoryIden::PrimaryCategory,
        ])
        .to_string(PostgresQueryBuilder);

    sqlx::query_as::<Postgres, PersonalFinanceCategory>(&query)
        .fetch_all(db)
        .await
        .unwrap()
}
