use ::Connector;
use ::types::*;

pub trait FormatColumn<'a> {

    fn wildcard(&self) -> &str where Self: Connector<'a> {
        "*"
    }

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
        self.for_table(table, self.wildcard())
    }
}

pub fn format_columns<'a>(connector: &Connector, columns: &TableColumns) -> String {
    let mut list = Vec::new();

    for (table, columns) in columns {
        //Reserve size for columns
        list.reserve(columns.len());

        for column in &columns.columns {
            list.push(
                FormatColumn::for_table(
                    connector,
                    &table,
                    &format_column(connector, &column)
                )
            );
        }
    }

    FormatColumn::list(connector, list)
}

pub fn format_column(connector: &Connector, column: &Column) -> String {

    match column {
        //Quote column without alias
        Column::Name(column) => FormatColumn::quote(connector, column),
        //Quote column with alias
        Column::Alias(column, alias) => FormatColumn::alias(
            connector,
            &FormatColumn::quote(connector, column),
            alias
        ),
        //Quote column for All (SELECT *)
        Column::All => FormatColumn::wildcard(connector).to_owned()
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
    use std::collections::HashMap;

    use ::hr::Hr;
    // use ::Connector;
    use ::types::*;

    use super::{
        FormatTable,
        FormatColumn
    };

    #[test]
    fn format_table_quoting(){
        let hr = Hr{};

        assert_eq!(
            FormatTable::quote(&hr, "hello"),
            "hello"
        );
    }

    #[test]
    fn format_table(){
        let hr = Hr{};
        let table = Table::Name("table_name");

        let format_table = ::format_table(&hr, &table);
        let should_be = String::from("table_name");
        assert_eq!(format_table, should_be);
    }

    #[test]
    fn format_table_alias(){
        let hr = Hr{};
        let table = Table::Alias("table_name", "tn");

        let format_table = ::format_table(&hr, &table);
        let should_be = String::from("table_name AS tn");
        assert_eq!(format_table, should_be);
    }

    #[test]
    fn format_tables(){
        let hr = Hr{};

        let mut tables = Vec::with_capacity(2);
        tables.push(Table::Name("table"));
        tables.push(Table::Alias("hello_world", "hw"));

        let from = From::List(tables);

        let format_tables = ::format_tables(&hr, &from);
        let should_be = String::from("table,\n\thello_world AS hw");

        assert_eq!(format_tables, should_be);
    }

    #[test]
    fn format_column_for_table(){
        let hr = Hr{};

        //No alias
        let table = Table::Name("my_table");
        let column = Column::Name("my_column");

        let format = FormatColumn::for_table(
            &hr,
            &table,
            &::format_column(&hr, &column)
        );
        let should_be = String::from("my_table.my_column");
        assert_eq!(format, should_be);

        //Table aliased
        let table = Table::Alias("my_table", "mt");
        let column = Column::Name("my_column");

        let format = FormatColumn::for_table(
            &hr,
            &table,
            &::format_column(&hr, &column)
        );
        let should_be = String::from("mt.my_column");
        assert_eq!(format, should_be);

        //Column aliased
        let table = Table::Name("my_table");
        let column = Column::Alias("my_column", "mc");

        let format = FormatColumn::for_table(
            &hr,
            &table,
            &::format_column(&hr, &column)
        );
        let should_be = String::from("my_table.my_column AS mc");
        assert_eq!(format, should_be);

        //Both aliased
        let table = Table::Alias("my_table", "mt");
        let column = Column::Alias("my_column", "mc");

        let format = FormatColumn::for_table(
            &hr,
            &table,
            &::format_column(&hr, &column)
        );
        let should_be = String::from("mt.my_column AS mc");
        assert_eq!(format, should_be);
    }

    #[test]
    fn format_column_quoting(){
        let hr = Hr{};

        assert_eq!(
            FormatColumn::quote(&hr, "hello"),
            "hello"
        );
    }

    #[test]
    fn format_column(){
        let hr = Hr{};
        let column = Column::Name("column_name");

        let format_column = ::format_column(&hr, &column);
        let should_be = String::from("column_name");
        assert_eq!(format_column, should_be);
    }

    #[test]
    fn format_column_alias(){
        let hr = Hr{};
        let column = Column::Alias("column_name", "cn");

        let format_column = ::format_column(&hr, &column);
        let should_be = String::from("column_name AS cn");
        assert_eq!(format_column, should_be);
    }

    #[test]
    fn format_columns(){
        let hr = Hr{};

        let table = Table::Alias("my_table", "t");

        let columns = Columns::new()
            .add("column")
            .add(Column::Alias("hello_world", "hw"));

        // let mut columns = Vec::with_capacity(2);
        // columns.push(Column::Name("column"));
        // columns.push(Column::Alias("hello_world", "hw"));
        //
        // let columns = Columns::List(columns);

        let mut tables_columns: TableColumns = HashMap::new();
        tables_columns.insert(&table, columns);

        let format_columns = ::format_columns(&hr, &tables_columns);
        let should_be = String::from("t.column,\n\tt.hello_world AS hw");

        assert_eq!(format_columns, should_be);
    }

}
