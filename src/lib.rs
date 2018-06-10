pub mod query;
pub mod formatters;
pub mod types;
pub mod hr;

use types::*;
use formatters::*;

pub struct Query<'q> {
    // where C: 'q + Connector<'q> {

    query_type: QueryType,
    connector: &'q Connector<'q>,

    tables: From<'q>,
    columns: Option<TableColumns<'q>>
}

pub trait Connector<'q> :
    FormatColumn<'q> + FormatTable<'q> {

    // -- Start queries

    /// Create Select request
    fn select<T: Into<Table<'q>>>(&'q self, table: T) -> Query where Self: Sized {
        Query::select(self, table.into())
    }

    /// Create Update Request

    fn update<T: Into<Table<'q>>>(&'q self, table: T) -> Query where Self: Sized {
        Query::update(self, table.into())
    }

    /// Create Delete resquest
    fn delete<T: Into<Table<'q>>>(&'q self, table: T) -> Query where Self: Sized {
        Query::delete(self, table.into())
    }

    // -- Print queries

    fn print_select(
        &self,
        columns: &str,
        tables: &str
    ) -> String;

    fn print_update(
        &self,
        table: &str,
        values: &str
    ) -> String;

    fn print_delete(
        &self,
        table: &str
    ) -> String;

    // -- HR (Human Readability)

    /// Activate human readability
    fn toggle_hr(&self) -> Result<bool, String> { Err(String::from("HR Toggle is not implemented")) }
    /// True if formatted query should be human readable
    fn hr(&self) -> bool { true }
}
