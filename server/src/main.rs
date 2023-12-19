use server::{make_server, settings::Settings, utils::get_config_directory};
use sqlx::postgres::PgPoolOptions;

/// Entrypoint
///
/// This function initializes the application and its components, setting up
/// telemetry, reading configuration, and starting the server.
///
/// # Flow:
/// 1. Retrieve the configuration directory.
/// 2. Load application settings from the configuration directory.
/// 3. Initialize metrics and telemetry subscribers.
/// 4. Bind to a TCP address for the server.
/// 5. Create the server instance with a graceful shutdown mechanism.
/// 6. Start the server and log if there's any failure in the process.
///
/// # Panics:
/// The function will panic if:
/// - The configuration cannot be loaded.
/// - The TCP listener cannot be bound to the specified address.
#[tokio::main]
async fn main() {
    let config_directory = get_config_directory();

    let config = Settings::new(&config_directory).expect("Failed to load configuration");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .unwrap_or_else(|_| panic!("Failed to bind {}:{}", config.host, config.port));

    let db = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await
        .unwrap();

    let server = make_server(listener, config.clone(), db);

    println!(
        "Starting up Toy Plaid API @ {}:{}",
        config.host, config.port
    );

    if let Err(e) = server.await {
        println!("Toy Plaid API failed to start: {:?}", e)
    }
}
