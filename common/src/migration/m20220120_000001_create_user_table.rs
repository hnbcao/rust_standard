use sea_orm::DbBackend;
use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let database_backend = manager.get_database_backend();
        let db = manager.get_connection();
        match database_backend {
            DbBackend::MySql => db.execute_unprepared(MYSQL_MIGRATION_UP_DDL).await?,
            DbBackend::Postgres => db.execute_unprepared(MYSQL_MIGRATION_UP_DDL).await?,
            DbBackend::Sqlite => db.execute_unprepared(MYSQL_MIGRATION_UP_DDL).await?,
        };
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let database_backend = manager.get_database_backend();
        let db = manager.get_connection();
        match database_backend {
            DbBackend::MySql => db.execute_unprepared(MYSQL_MIGRATION_DOWN_DDL).await?,
            DbBackend::Postgres => db.execute_unprepared(MYSQL_MIGRATION_DOWN_DDL).await?,
            DbBackend::Sqlite => db.execute_unprepared(MYSQL_MIGRATION_DOWN_DDL).await?,
        };
        Ok(())
    }
}

const MYSQL_MIGRATION_UP_DDL: &str = r#"CREATE TABLE IF NOT EXISTS `user` (
  `id` int(11) NOT NULL,
  `username` varchar(255) DEFAULT NULL,
  `password` varchar(255) DEFAULT NULL,
  `email` varchar(255) DEFAULT NULL,
  `phone` varchar(20) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `pk_phone` (`phone`) USING BTREE
);

CREATE TABLE IF NOT EXISTS `role` (
  `id` int(11) NOT NULL,
  `name` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `user_role` (
  `id` int(11) NOT NULL,
  `user_id` int(11) DEFAULT NULL,
  `role_id` int(11) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `pk_user_id_role_id` (`user_id`,`role_id`) USING BTREE
);"#;

const MYSQL_MIGRATION_DOWN_DDL: &str = r#"DROP TABLE IF EXISTS `user_role`;
DROP TABLE IF EXISTS `role`;
DROP TABLE IF EXISTS `user`;"#;