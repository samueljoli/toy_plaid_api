use sea_query::{PostgresQueryBuilder, Query};
use sqlx::Postgres;

use super::models::{PersonalFinanceCategory, PersonalFinanceCategoryIden};

pub async fn select_all_categories(
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Vec<PersonalFinanceCategory> {
    let query = Query::select()
        .from(PersonalFinanceCategoryIden::Table)
        .columns(vec![
            PersonalFinanceCategoryIden::Id,
            PersonalFinanceCategoryIden::Detailed,
            PersonalFinanceCategoryIden::PrimaryCategory,
        ])
        .to_string(PostgresQueryBuilder);

    sqlx::query_as::<Postgres, PersonalFinanceCategory>(&query)
        .fetch_all(&mut **trx)
        .await
        .unwrap()
}
