use database::Database;
use lazy_static::lazy_static;

mod database;
mod errors;
mod fixtures;
pub mod vm;

lazy_static! {
    static ref DATABASE: Database = Database::default();
}
