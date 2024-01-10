use celery::prelude::*;
use serde_json::{Map, Value};

#[celery::task]
pub async fn fire_webhook(item_id: i32, url: String) -> TaskResult<()> {
    println!("Firing webhook for item_id: {}", &item_id);

    let mut payload = Map::new();
    payload.insert(
        "webhook_type".to_string(),
        Value::String("TRANSACTIONS".to_string()),
    );
    payload.insert(
        "webhook_code".to_string(),
        Value::String("SYNC_UPDATES_AVAILABLE".to_string()),
    );
    payload.insert("item_id".to_string(), Value::String(item_id.to_string()));
    payload.insert("initial_update_complete".to_string(), Value::Bool(true));
    payload.insert("historical_update_complete".to_string(), Value::Bool(false));
    payload.insert(
        "environment".to_string(),
        Value::String("production".to_string()),
    );

    let client = reqwest::Client::new();

    let res = client.post(url).json(&payload).send().await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error: {}", e);
            Ok(())
        }
    }
}
