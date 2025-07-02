use serde::{Serialize, Deserialize};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: Option<i64>,
    pub title: String,
    pub completed: bool,
}

pub async fn init_db() -> Pool<Sqlite> {
    let config = Config::init();
    if !Sqlite::database_exists(&config.database_url).await.unwrap_or(false) {
        println!("Creating database {}", &config.database_url);
        match Sqlite::create_database(&config.database_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(&config.database_url).unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migration = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations) 
        .await
        .unwrap()
        .run(&pool)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    pool
}

