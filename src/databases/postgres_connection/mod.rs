use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub async fn start_connection() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres.");

    if let Err(e) = sqlx::migrate!("./src/databases/postgres_connection/migrations")
        .run(&pool)
        .await
    {
        println!("Error running migrations: {:?}", e);
    } else {
        println!("Migrations run successfully");
    }

    pool
}
