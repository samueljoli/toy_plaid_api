use server::{make_server, settings::Settings, utils::get_config_directory};
use sqlx::postgres::PgPoolOptions;
use tracing::{error, info, Level};

/// Entrypoint
#[tokio::main]
async fn main() {
    let config_directory = get_config_directory();

    let config = Settings::new(&config_directory).expect("Failed to load configuration");

    let subscriber = tracing_subscriber::fmt()
        // Be less verbose
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(true)
        // Add filter level
        .with_max_level(Level::DEBUG)
        // Build the subscriber
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind {}:{}", config.host, config.port));

    let db = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let server = make_server(listener, config.clone(), db).await;

    info!(
        "Starting up Toy Plaid API @ {}:{}",
        config.host, config.port
    );

    if let Err(e) = server.await {
        error!("Toy Plaid API failed to start: {:?}", e)
    }
}
