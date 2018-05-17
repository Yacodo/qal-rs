use std::fmt;

use ::*;
use types::*;

impl<'q, C> Query<'q, C>
    where C: 'q + Connector<'q> {

    /// Create a new Select Query
    pub fn select<T: Into<Table<'q>>>(connector: &'q C, table: T) -> Query<'q, C> {
        let mut query = Query::<C> {
            query_type: QueryType::Select,
            connector: connector,
            tables: From::new_list()
        };

        query.from(table.into());

        query
    }

    /// Create a new Update Query
    pub fn update<T: Into<Table<'q>>>(connector: &'q C, table: T) -> Query<'q, C> {
        let mut query = Query::<C> {
            query_type: QueryType::Update,
            connector: connector,
            tables: From::only_one(table.into())
        };

        query
    }

    /// Create a new Delete Query
    pub fn delete<T: Into<Table<'q>>>(connector: &'q C, table: T) -> Query<'q, C> {
        let mut query = Query::<C> {
            query_type: QueryType::Delete,
            connector: connector,
            tables: From::only_one(table.into())
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

    pub fn from<T: Into<Table<'q>>>(&mut self, table: T) -> bool {
        match self.tables {
            From::List(ref mut map) => {
                (*map).push(table.into());
            },
            _ => return false
        }

        true
    }

    /// # Formatting

    fn format_select(&self) -> String {
        self.connector.print_select(
            String::from("*"),
            formatters::format_tables(self.connector, &self.tables)
        )
    }

    fn format_update(&self) -> String {
        self.connector.print_update(
            formatters::format_tables(self.connector, &self.tables),
            String::from("1=1")
        )
    }

    fn format_delete(&self) -> String {
        self.connector.print_delete(
            formatters::format_tables(self.connector, &self.tables)
        )
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
        })
    }
}
