use sea_orm_migration::sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub db: DatabaseConnection,
}

impl AppContext {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { db: conn }
    }
}
