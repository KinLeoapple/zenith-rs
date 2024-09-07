pub use sea_orm_migration::prelude::*;

mod user;
mod category;
mod blog;
mod image;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(user::Migration),
            Box::new(category::Migration),
            Box::new(blog::Migration),
            Box::new(image::Migration),
        ]
    }
}
