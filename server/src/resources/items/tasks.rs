use celery::prelude::*;

#[celery::task]
pub fn add(x: i32, y: i32) -> TaskResult<i32> {
    println!("Adding {} + {}", x, y);
    Ok(x + y)
}
