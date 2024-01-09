use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::Postgres;

use super::models::{Institution, InstitutionIden};

pub async fn select_institution_by_name(
    name: &str,
    trx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Institution {
    let query = Query::select()
        .columns(vec![InstitutionIden::Id, InstitutionIden::Name])
        .from(InstitutionIden::Table)
        .and_where(Expr::col(InstitutionIden::Name).eq(name))
        .to_string(PostgresQueryBuilder)
        .to_owned();

    sqlx::query_as::<Postgres, Institution>(&query)
        .fetch_one(&mut **trx)
        .await
        .unwrap()
}
