use celery::prelude::*;

use serde_derive::Deserialize;

#[celery::task]
pub fn add(x: i32, y: i32) -> TaskResult<i32> {
    println!("Adding {} + {}", x, y);
    Ok(x + y)
}

#[derive(Deserialize, Debug, Clone)]
struct Row<'a> {
    primary: &'a str,
    detailed: &'a str,
    description: &'a str,
}

// pub fn categories_name_to_id_map() {
//     let mut path = std::env::current_dir().unwrap();
//
//     path.pop();
//
//     path.push("scripts");
//
//     let full_path = path.join("personal_finance_categories.csv");
//
//     let full_path = full_path.to_str().unwrap();
//
//     let mut map: HashMap<&str, String> = HashMap::new();
//
//     let mut reader = Reader::from_path(full_path).unwrap();
//
//     for record in reader.records() {
//         // let row: Row = record.deserialize(None)?;
//         let record = record.unwrap();
//
//         let row: Row = record.deserialize(None).unwrap();
//
//         map.insert(
//
//         )
//     }
// }

#[celery::task]
pub fn build_integration() -> TaskResult<()> {
    // let map = categories_name_to_id_map(db);
    // create accounts
    // create categories_map
    // create transactions
    Ok(())
}
