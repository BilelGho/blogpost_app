use std::fs;

pub async fn db_connect() -> sqlx::sqlite::SqlitePool {
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();

    assert_eq!(key, "DATABASE_URL");

    println!("database_url: {}", database_url);

    sqlx::sqlite::SqlitePool::connect(database_url).await.unwrap()
}