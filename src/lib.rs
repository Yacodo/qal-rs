pub mod query;
pub mod formatters;
pub mod types;
pub mod hr;

use types::*;
use formatters::*;

pub struct Query<'q, C>
    where C: 'q + Connector<'q> {

    query_type: QueryType,
    connector: &'q C,

    tables: From<'q>
}

pub trait Connector<'q> : FormatColumn<'q> + FormatTable<'q> {

    // -- Start queries

    /// Create Select request
    fn select<T: Into<Table<'q>>>(&'q self, table: T) -> Query<Self> where Self: Sized {
        Query::select(self, table.into())
    }

    /// Create Update Request

    fn update<T: Into<Table<'q>>>(&'q self, table: T) -> Query<Self> where Self: Sized {
        Query::update(self, table.into())
    }

    /// Create Delete resquest
    fn delete<T: Into<Table<'q>>>(&'q self, table: T) -> Query<Self> where Self: Sized {
        Query::delete(self, table.into())
    }

    // -- Print queries

    fn print_select(
        &self,
        columns: String,
        tables: String
    ) -> String;

    fn print_update(
        &self,
        table: String,
        values: String
    ) -> String;

    fn print_delete(
        &self,
        table: String
    ) -> String;

    // -- HR (Human Readability)

    /// Activate human readability
    fn toggle_hr(&self) -> Result<bool, String> { Err(String::from("HR Toggle is not implemented")) }
    /// True if formatted query should be human readable
    fn hr(&self) -> bool { false }
}
