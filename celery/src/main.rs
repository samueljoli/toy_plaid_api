#![allow(non_upper_case_globals)]

use exitfailure::ExitFailure;
use server::resources::items::tasks::add;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let celery_app = celery::app!(
        broker = RedisBroker { std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into()) },
        tasks = [add],
        task_routes = [
            "*" => "celery",
        ],
        prefetch_count = 2
    ).await?;

    celery_app.consume().await?;

    Ok(())
}
