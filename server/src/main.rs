use server::{make_server, settings::Settings, utils::get_config_directory};
use sqlx::postgres::PgPoolOptions;

/// Entrypoint
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
        .expect("Failed to connect to database");

    let server = make_server(listener, config.clone(), db).await;

    println!(
        "Starting up Toy Plaid API @ {}:{}",
        config.host, config.port
    );

    if let Err(e) = server.await {
        println!("Toy Plaid API failed to start: {:?}", e)
    }
}
