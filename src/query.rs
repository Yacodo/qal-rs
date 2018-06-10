use std::{
    fmt,
    collections::HashMap,
};

use ::*;
use types::*;

impl<'q> Query<'q> {
    // where C: 'q + Connector<'q> {

    /// Create a new Select Query
    pub fn select<T: Into<Table<'q>>>(connector: &'q Connector<'q>, table: T) -> Self {
        let mut query = Query {
            query_type: QueryType::Select,
            connector: connector,

            tables: From::new_list(),
            columns: Some(HashMap::new())
        };

        let table = table.into();

        query.from(table);

        query
    }

    /// Create a new Update Query
    pub fn update<T: Into<Table<'q>>>(connector: &'q Connector<'q>, table: T) -> Self {
        let query = Query {
            query_type: QueryType::Update,
            connector: connector,

            tables: From::one(table.into()),
            columns: None
        };

        query
    }

    /// Create a new Delete Query
    pub fn delete<T: Into<Table<'q>>>(connector: &'q Connector<'q>, table: T) -> Self {
        let query = Query {
            query_type: QueryType::Delete,
            connector: connector,

            tables: From::one(table.into()),
            columns: None
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

    /// # Set Datas

    pub fn columns<T: Into<Table<'q>>, C: 'q + Into<Columns<'q>>>
        (mut self, table: T, columns: C) -> Self {

        // let table = table.into();
        //
        // match self.tables {
        //     From::List(ref mut tables) => {
        //         //Add columns
        //         let mut lc = self.columns.unwrap();
        //         lc.insert(
        //             &table,
        //             columns.into()
        //         );
        //
        //         //Add to From table list
        //         (*tables).push(table);
        //     },
        //     _ => {}
        // }

        self
    }

    pub fn from<T: Into<Table<'q>>>
        (&mut self, table: T) -> Option<&'q mut Columns<'q>> {



        None
    }

    /// # Formatting

    fn format_select(&self) -> String {
        self.connector.print_select(
            "*",
            &formatters::format_tables(self.connector, &self.tables)
        )
    }

    fn format_update(&self) -> String {
        self.connector.print_update(
            &formatters::format_tables(self.connector, &self.tables),
            "1=1"
        )
    }

    fn format_delete(&self) -> String {
        self.connector.print_delete(
            &formatters::format_tables(self.connector, &self.tables)
        )
    }

}

impl<'q> fmt::Display for Query<'q> {
    // where C: 'q + Connector<'q> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use QueryType::*;

        write!(f, "{}", match self.query_type {
            Select => self.format_select(),
            Update => self.format_update(),
            Delete => self.format_delete(),
        })
    }
}

#[cfg(test)]
mod test {
    use ::hr::Hr;
    use ::{
        Connector,
        // Query,
    };

    #[test]
    fn create_select(){
        let hr = Hr{};
        let mut select = hr.select("my_table");

        assert!(select.is_select(), "should be QueryType::Select");
        assert!(select.from("").is_some(), "should be able to add table to Select");
    }

    #[test]
    fn create_update(){
        let hr = Hr{};
        let mut update = hr.update("my_table");

        assert!(update.is_update(), "should be QueryType::Update");
        assert!(!update.from("").is_none(), "can't add a new table to an Update");
    }

    #[test]
    fn create_delete(){
        let hr = Hr{};
        let mut delete = hr.delete("my_table");

        assert!(delete.is_delete(), "should be QueryType::Delete");
        assert!(!delete.from("").is_none(), "can't add a new table to Delete");
    }

    #[test]
    fn is_hr(){
        let hr = Hr{};
        assert!(!hr.hr());
    }

    #[test]
    fn toggle_hr(){
        let hr = Hr{};
        assert!(hr.toggle_hr().is_err());
    }
}
