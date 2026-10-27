mod error;
mod get_page;
mod page;
mod table;

pub use error::{PageError, TableError};
pub use get_page::get_page;
pub use page::page;
pub use table::{json_as_table, print_json_as_table};
