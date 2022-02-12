use sea_orm::DatabaseConnection;

pub type Pool = DatabaseConnection;

pub async fn get_db_pool() -> Pool {
    
    let mut opt = sea_orm::ConnectOptions::new(std::env::var("DATABASE_URL").unwrap());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(std::time::Duration::from_secs(8))
        .idle_timeout(std::time::Duration::from_secs(8))
        .max_lifetime(std::time::Duration::from_secs(8))
        .sqlx_logging(true);
    let db = sea_orm::Database::connect(opt).await.unwrap();
    
    return db;
}