use database::Database;
use lazy_static::lazy_static;

mod database;
mod errors;
pub mod vm;

lazy_static! {
    static ref DATABASE: Database = Database::default();
}
