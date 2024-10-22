use sea_orm::{DatabaseConnection, DbErr};
use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod m20220120_000001_create_user_table;

pub async fn migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await?;
    Ok(())
}

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220120_000001_create_user_table::Migration),
        ]
    }
}

#[cfg(test)]
pub mod tests {
    use sea_orm::Database;
    use super::*;
    pub async fn setup() -> DatabaseConnection {
        let opt = sea_orm::ConnectOptions::new("mysql://root:302@Segma@10.73.13.51:3307/rust_standard");
        Database::connect(opt).await.expect("Database connection error")
    }

    #[tokio::test]
    pub async fn test_migration_up() {
        let db = setup().await;
        Migrator::up(&db, None).await.expect("Migrator::up");
    }

    #[tokio::test]
    pub async fn test_migration_down() {
        let db = setup().await;
        Migrator::down(&db, None).await.expect("Migrator::down");
    }
}