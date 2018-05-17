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
    use ::hr::Hr;
    // use ::Connector;
    use ::types::*;

    #[test]
    fn format_table(){
        let c = Hr{};
        let table = Table::Name("table_name");

        let format_table = ::format_table(&c, &table);
        let should_be = String::from("\"table_name\"");
        assert_eq!(format_table, should_be);
    }

    #[test]
    fn format_alias(){
        let c = Hr{};
        let table = Table::Alias("table_name", "tn");

        let format_table = ::format_table(&c, &table);
        let should_be = String::from("\"table_name\" AS tn");
        assert_eq!(format_table, should_be);
    }

    #[test]
    fn format_tables(){
        let c = Hr{};

        let mut tables = Vec::with_capacity(2);
        tables.push(Table::Name("table"));
        tables.push(Table::Alias("hello_world", "hw"));

        let from = From::List(tables);

        let format_tables = ::format_tables(&c, &from);
        let should_be = String::from("\"table\", \"hello_world\" AS hw");

        assert_eq!(format_tables, should_be);
    }

}
