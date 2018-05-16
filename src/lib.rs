pub mod query;
pub mod formatters;
pub mod types;

use types::*;
use formatters::*;

pub struct Query<'q, C>
    where C: 'q + Connector<'q> {

    query_type: QueryType,
    connector: &'q C,
    table: &'q str,

    tables: From<'q>
}

pub trait Connector<'q> : FormatColumn<'q> + FormatTable<'q> {

    fn select(&'q self, table: &'q str) -> Query<Self> where Self: Sized {
        Query::select(self, table)
    }

    fn update(&'q self, table: &'q str) -> Query<Self> where Self: Sized {
        Query::update(self, table)
    }

    fn delete(&'q self, table: &'q str) -> Query<Self> where Self: Sized {
        Query::delete(self, table)
    }

    fn print_select(
        &self,
        columns: &str,
        tables: &str
    ) -> String;

    fn print_update(&self, table: &str, values: &str) -> String;
    fn print_delete(&self, table: &str) -> String;

    // -- HR (Human Readability)

    /// Activate human readability
    fn toggle_hr(&self) -> Result<bool, String> { Err(String::from("HR Toggle is not implemented")) }
    /// True if formatted query should be human readable
    fn hr(&self) -> bool { true }
}
