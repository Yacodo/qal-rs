use ::Connector;
use ::types::*;

pub trait FormatColumn<'a> {
    fn for_table(&self, table: &Table, column: &str) -> String where Self: Connector<'a> {
        format!("{}.{}", table.identifier(), column)
    }

    fn quote(&self, column: &str)              -> String where Self: Connector<'a> {
        format!("{}", column)
    }

    fn alias(&self, column: &str, alias: &str) -> String where Self: Connector<'a> {
        format!("{} AS {}", column, alias)
    }

    fn list(&self, columns: Vec<String>)       -> String where Self: Connector<'a> {
        columns.join(",\n\t")
    }

    fn all(&self, table: &Table) -> String where Self: Connector<'a> {
        Self::for_table(self, table, "*")
    }
}

pub fn format_columns<'a>(connector: &Connector, columns: &TableColumns) -> String {
    let mut list = Vec::new();

    for (table, columns) in columns {
        //Reserve size for columns
        list.reserve(columns.len());

        match columns {
            Columns::List(columns) => {
                for column in columns {
                    list.push(format_column(connector, &table, column));
                }
            },
            Columns::All => list.push(FormatColumn::all(connector, &table))
        };
    }

    FormatColumn::list(connector, list)
}

pub fn format_column(connector: &Connector, table: &Table, column: &Column) -> String {

    match column {
        //Quote column without alias
        Column::Name(column) => FormatColumn::quote(connector, column),
        //Quote column with alias
        Column::Alias(column, alias) => FormatColumn::alias(
            connector,
            &FormatColumn::quote(connector, column),
            alias
        )
    }

}


pub trait FormatTable<'a> {

    fn quote(&self, table: &str)                -> String where Self: Connector<'a> {
        format!("{}", table)
    }

    fn alias(&self, table: &str, alias: &str)   -> String where Self: Connector<'a> {
        format!("{} AS {}", table, alias)
    }


    fn list(&self, tables: Vec<String>)         -> String where Self: Connector<'a> {
        tables.join(",\n\t")
    }

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
    use std::rc::Rc;

    use ::hr::Hr;
    // use ::Connector;
    use ::types::*;
    use super::FormatTable;

    #[test]
    fn format_table_quoting(){
        let c = Hr{};

        assert_eq!(
            FormatTable::quote(&c, "hello"),
            "hello"
        );
    }

    #[test]
    fn format_table(){
        let c = Hr{};
        let table = Table::Name("table_name");

        let format_table = ::format_table(&c, &table);
        let should_be = String::from("table_name");
        assert_eq!(format_table, should_be);
    }

    #[test]
    fn format_alias(){
        let c = Hr{};
        let table = Table::Alias("table_name", "tn");

        let format_table = ::format_table(&c, &table);
        let should_be = String::from("table_name AS tn");
        assert_eq!(format_table, should_be);
    }

    #[test]
    fn format_tables(){
        let c = Hr{};

        let mut tables = Vec::with_capacity(2);
        tables.push(Rc::new(Table::Name("table")));
        tables.push(Rc::new(Table::Alias("hello_world", "hw")));

        let from = From::List(tables);

        let format_tables = ::format_tables(&c, &from);
        let should_be = String::from("table,\n\thello_world AS hw");

        assert_eq!(format_tables, should_be);
    }

}
