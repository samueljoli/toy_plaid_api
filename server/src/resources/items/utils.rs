use csv::ReaderBuilder;
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::types::chrono::{NaiveDate, NaiveDateTime};

fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
}

// Custom deserializer for NaiveDateTime
fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ").map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionRow {
    pub name: String,
    pub merchant_name: String,
    pub payment_channel: String,
    pub amount: String,
    #[serde(deserialize_with = "deserialize_naive_date")]
    pub date: NaiveDate,
    #[serde(deserialize_with = "deserialize_naive_datetime")]
    pub datetime: NaiveDateTime,
    pub personal_finance_category: String,
}

pub fn get_transactions_from_csv() -> Vec<TransactionRow> {
    let mut path = std::env::current_dir().unwrap();

    let mut results: Vec<TransactionRow> = vec![];

    path.push("scripts");

    let full_path = path.join("checkings_transactions.csv");

    let full_path = full_path.to_str().unwrap();

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_path(full_path)
        .unwrap();

    for record in reader.records() {
        let record = record.unwrap();

        let row: TransactionRow = record.deserialize(None).unwrap();

        results.push(row)
    }

    results
}
