use sea_query::{Expr, PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres};

use super::models::{Institution, InstitutionIden};

pub async fn select_institution_by_name(name: &str, db: &Pool<Postgres>) -> Institution {
    let query = Query::select()
        .columns(vec![InstitutionIden::Id, InstitutionIden::Name])
        .from(InstitutionIden::Table)
        .and_where(Expr::col(InstitutionIden::Name).eq(name))
        .to_string(PostgresQueryBuilder)
        .to_owned();

    sqlx::query_as::<Postgres, Institution>(&query)
        .fetch_one(db)
        .await
        .unwrap()
}
