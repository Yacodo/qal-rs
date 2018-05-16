use std::fmt;

use ::*;
use types::*;

impl<'q, C> Query<'q, C>
    where C: 'q + Connector<'q> {

    /// Create a new Select Query
    pub fn select(connector: &'q C, table: &'q str) -> Query<'q, C> {
        let query = Query::<C> {
            query_type: QueryType::Select,
            connector: connector,
            table,
            tables: From::new_list()
        };

        query
    }

    /// Create a new Update Query
    pub fn update(connector: &'q C, table: &'q str) -> Query<'q, C> {
        let query = Query::<C> {
            query_type: QueryType::Update,
            connector: connector,
            table,
            tables: From::only_one(table)
        };

        query
    }

    /// Create a new Delete Query
    pub fn delete(connector: &'q C, table: &'q str) -> Query<'q, C> {
        let query = Query::<C> {
            query_type: QueryType::Delete,
            connector: connector,
            table,
            tables: From::only_one(table)
        };

        query
    }

    /// # Query Types

    /// Check if the query is QueryType::Select
    pub fn is_select(&self) -> bool {
        self.query_type == QueryType::Select
    }

    /// Check if the query is QueryType::Update
    pub fn is_update(&self) -> bool {
        self.query_type == QueryType::Update
    }

    /// Check if the query is QueryType::Delete
    pub fn is_delete(&self) -> bool {
        self.query_type == QueryType::Delete
    }

    /// # Query Datas

    pub fn tables(&self) -> &From<'q> {
        &self.tables
    }

    /// # Formatting

    fn format_select(&self) -> String {
        self.connector.print_select(
            "*",
            self.table
        )
    }

    fn format_update(&self) -> String {
        self.connector.print_update(self.table, "1=1")
    }

    fn format_delete(&self) -> String {
        self.connector.print_delete(self.table)
    }

}

impl<'q, C> fmt::Display for Query<'q, C>
    where C: 'q + Connector<'q> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use QueryType::*;

        write!(f, "{}", match self.query_type {
            Select => self.format_select(),
            Update => self.format_update(),
            Delete => self.format_delete(),
            _ => String::from("")
        })
    }
}
