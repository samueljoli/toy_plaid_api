use axum::{
    routing::{get, IntoMakeService},
    serve::Serve,
    Router,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

pub mod settings;

pub mod resources;

pub mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    config: settings::Settings,
    db: Pool<Postgres>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        resources::transactions::router::get_transaction_by_id,
    ),
    tags(
        (name = "todo", description = "Todo items management API")
    )
)]
struct ApiDoc;

fn health_check_router() -> Router<AppState> {
    Router::new().route("/health", get(|| async { Ok::<_, ()>(()) }))
}

fn make_app(config: settings::Settings, db: Pool<Postgres>) -> Router {
    Router::new()
        .merge(health_check_router())
        .merge(resources::transactions::router::api())
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .with_state(AppState { config, db })
}

pub fn make_server(
    listener: TcpListener,
    config: settings::Settings,
    db: Pool<Postgres>,
) -> Serve<IntoMakeService<Router>, Router> {
    axum::serve(listener, make_app(config, db).into_make_service())
}
