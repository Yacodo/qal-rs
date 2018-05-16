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
        let mut columns: String = columns.join(
            if self.hr() {
                ", "
            }else{
                ",\r\n\t"
            }
        );

        if self.hr() {
            columns.insert_str(0, "`\r")
        }

        columns
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
        let mut tables: String = tables.join(
            if self.hr() {
                ", "
            }else{
                ",\r\n\t"
            }
        );

        if self.hr() {
            tables.insert_str(0, "`\r")
        }

        tables
    }
}

pub fn format_columns<'a>(connector: &Connector, from: &From<'a>) -> String {
    let result = String::new();

    

    result
}

pub fn format_tables<'a>(connector: &Connector, from: &From<'a>) -> String {
    let mut list = Vec::with_capacity(from.len());

    match from {
        From::List(map) => {
            for (table, _) in map.iter() {
                list.push(format_table(connector, table));
            }
        },
        From::One(table, _) => {
            list.push(format_table(connector, table));
        }
    };

    FormatTable::list(connector, list)
}

pub fn format_table<'a>(connector: &Connector, table: &Table) -> String {

    match table {
        Table::Name(table) => FormatTable::quote(connector, table),
        Table::Alias(table, alias) => FormatTable::alias(
            connector,
            &FormatTable::quote(connector, table),
            alias
        )
    }

}
