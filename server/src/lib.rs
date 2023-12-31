use axum::{
    routing::{get, IntoMakeService},
    serve::Serve,
    Router,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

pub mod settings;

pub mod resources;

pub mod utils;

use resources::transactions;

use resources::accounts;

use resources::companies;

#[derive(Debug, Clone)]
pub struct AppState {
    config: settings::Settings,
    db: Pool<Postgres>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        accounts::router::get_account_by_id,
        accounts::router::get_all_accounts,
        companies::router::get_company_by_id,
        transactions::router::get_all_transactions,
        transactions::router::get_transaction_by_id,
    ),
    components(
        schemas(transactions::models::Transaction),
        schemas(accounts::models::Account),
        schemas(companies::models::Company),
    ),
    tags(
        (name = "🧸 Toy Plaid API", description = "A toy implementation of the Plaid API")
    )
)]
struct ApiDoc;

fn make_main_router(config: settings::Settings, db: Pool<Postgres>) -> Router {
    Router::new()
        .merge(Router::new().route("/health", get(|| async { Ok::<_, ()>(()) })))
        .merge(resources::transactions::router::api())
        .merge(resources::accounts::router::api())
        .merge(resources::companies::router::api())
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .layer(ServiceBuilder::new().layer(CorsLayer::new().allow_origin(Any)))
        .with_state(AppState { config, db })
}

pub fn make_server(
    listener: TcpListener,
    config: settings::Settings,
    db: Pool<Postgres>,
) -> Serve<IntoMakeService<Router>, Router> {
    axum::serve(listener, make_main_router(config, db).into_make_service())
}
