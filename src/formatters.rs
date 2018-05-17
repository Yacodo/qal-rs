use ::Connector;
use ::types::*;

/// TODO Should not be designed with PG in mind or quoting should be simplified for connector
pub trait FormatColumn<'a> {
    fn for_table(&self, table: &str, column: &str) -> String where Self: Connector<'a> {
        format!("{}.{}", table, column)
    }

    fn quote(&self, column: &str)              -> String where Self: Connector<'a> {
        format!("\"{}\"", column)
    }

    fn alias(&self, column: &str, alias: &str) -> String where Self: Connector<'a> {
        format!("{} AS {}", column, alias)
    }

    fn list(&self, columns: Vec<String>)       -> String where Self: Connector<'a> {
        columns.join(", ")
    }
}

pub trait FormatTable<'a> {

    fn quote(&self, table: &str)                -> String where Self: Connector<'a> {
        format!("\"{}\"", table)
    }

    fn alias(&self, table: &str, alias: &str)   -> String where Self: Connector<'a> {
        format!("{} AS {}", table, alias)
    }


    fn list(&self, tables: Vec<String>)         -> String where Self: Connector<'a> {
        tables.join(", ")
    }
}

pub fn format_columns<'a>(connector: &Connector, from: &From<'a>) -> String {
    let result = String::new();



    result
}

pub fn format_tables<'a>(connector: &Connector, from: &From<'a>) -> String {
    let mut list = Vec::with_capacity(from.len());

    match from {
        From::List(tables) => {
            for table in tables {
                list.push(format_table(connector, &table));
            }
        },
        From::One(table) => list.push(format_table(connector, &table))
    };

    FormatTable::list(connector, list)
}


pub fn format_table(connector: &Connector, table: &Table) -> String {

    match table {
        //Quote table without alias
        Table::Name(table) => FormatTable::quote(connector, table),
        //Quote table with alias
        Table::Alias(table, alias) => FormatTable::alias(
            connector,
            &FormatTable::quote(connector, table),
            alias
        )
    }

}

#[cfg(test)]
mod test {
    use ::Connector;
    use ::types::*;

    //Connector Struct / Imp is down there

    #[test]
    fn format_table(){
        let c = Fake{};
        let table = Table::Name("table_name");

        let format_table = ::format_table(&c, &table);
        let expect = String::from("\"table_name\"");
        assert_eq!(format_table, expect);
    }

    #[test]
    fn format_alias(){
        let c = Fake{};
        let table = Table::Alias("table_name", "tn");

        let format_table = ::format_table(&c, &table);
        let expect = String::from("\"table_name\" AS tn");
        assert_eq!(format_table, expect);
    }

    pub struct Fake{}
    impl<'q> Connector<'q> for Fake{
        fn print_select(&self, columns: String, tables: String) -> String {
            format!("SELECT {} FROM {}", columns, tables)
        }

        fn print_update(&self, table: String, values: String) -> String {
            format!("UPDATE {} SET {}", table, values)
        }

        fn print_delete(&self, table: String) -> String {
            format!("DELETE FROM {}", table)
        }
    }
    impl<'a> ::FormatColumn<'a> for Fake {}
    impl<'a> ::FormatTable<'a> for Fake {}

}
