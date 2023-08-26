pub use sea_orm_migration::prelude::*;
pub mod config;

mod m20230824_110340_create_user_table;
mod m20230824_220114_create_image_table;
mod m20230825_171205_alter_image_add_public_id;
mod m20230826_123626_create_profile_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230824_110340_create_user_table::Migration),
            Box::new(m20230824_220114_create_image_table::Migration),
            Box::new(m20230825_171205_alter_image_add_public_id::Migration),
            Box::new(m20230826_123626_create_profile_table::Migration),
        ]
    }
}
