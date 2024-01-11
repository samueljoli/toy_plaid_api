use std::sync::Arc;

use axum::{
    routing::{get, IntoMakeService},
    serve::Serve,
    Router,
};
use celery::Celery;
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

use resources::items;

#[derive(Clone)]
pub struct AppState {
    config: settings::Settings,
    db: Pool<Postgres>,
    celery_app: Arc<Celery>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        accounts::router::get_account_by_id,
        accounts::router::get_all_accounts,
        companies::router::get_company_by_id,
        transactions::router::get_all_transactions,
        transactions::router::get_transaction_by_id,
        items::router::post_item,
    ),
    components(
        schemas(transactions::models::Transaction),
        schemas(accounts::models::Account),
        schemas(companies::models::Company),
        schemas(items::models::Item),
    ),
    info(
        title = "🧸 Toy Plaid API",
        description = "
            This is a toy API that mimics the Plaid API. It is not intended to be used in production.
            It is intended to be used as a learning tool for the Rust programming language.
"
    )
)]
struct ApiDoc;

pub async fn make_celery_app() -> Arc<Celery> {
    // Fail fast and loudly if we can't connect to redis
    celery::app!(
        broker = RedisBroker { std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into()) },
        tasks = [items::tasks::fire_webhook],
        task_routes = [
            "*" => "celery",
        ],
        prefetch_count = 2
    ).await.expect("Unable to create celery app")
}

async fn core_router(config: settings::Settings, db: Pool<Postgres>) -> Router {
    let celery_app = make_celery_app().await;

    Router::new()
        .merge(Router::new().route("/health", get(|| async { Ok::<_, ()>(()) })))
        .merge(resources::transactions::router::api())
        .merge(resources::accounts::router::api())
        .merge(resources::companies::router::api())
        .merge(resources::items::router::api())
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .layer(ServiceBuilder::new().layer(CorsLayer::new().allow_origin(Any)))
        .with_state(AppState {
            config,
            db,
            celery_app,
        })
}

pub async fn make_server(
    listener: TcpListener,
    config: settings::Settings,
    db: Pool<Postgres>,
) -> Serve<IntoMakeService<Router>, Router> {
    axum::serve(listener, core_router(config, db).await.into_make_service())
}
