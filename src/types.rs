use std::{
    cmp,
    iter,
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Debug, PartialEq)]
pub enum QueryType {
    Select,
    Update,
    Delete,
}

#[derive(Debug, PartialEq)]
pub enum JoinType {
    Join,
    LeftJoin,
    RightJoin,
    //TODO MOAR
}

/// # Table

#[derive(Debug, Hash)]
pub enum Table<'a> {
    Alias(&'a str, &'a str),
    Name(&'a str)
}

impl<'a> cmp::PartialEq for Table<'a> {
    fn eq(&self, other: &Table<'a>) -> bool {
        if let Table::Alias(table, alias) = self {
            match other {
                Table::Alias(other_table, other_alias) => {
                    return table == other_table
                    && alias == other_alias
                },
                _ => return false
            }
        }

        if let Table::Name(table) = self {
            match other {
                Table::Name(other_table) => return table == other_table,
                _ => return false
            }
        }

        //TODO: is there a better way
        //Forced result since we use if let
        false
    }
}

impl<'a> cmp::Eq for Table<'a> {}

/// # Column

#[derive(Debug, Hash)]
pub enum Column<'a> {
    Alias(&'a str, &'a str),
    Name(&'a str)
}

impl<'a> cmp::PartialEq for Column<'a> {
    fn eq(&self, other: &Column<'a>) -> bool {
        if let Column::Alias(column, alias) = self {
            match other {
                Column::Alias(other_column, other_alias) => {
                    return column == other_column
                    && alias == other_alias
                },
                _ => return false
            }
        }

        if let Column::Name(column) = self {
            match other {
                Column::Name(other_column) => return column == other_column,
                _ => return false
            }
        }

        //TODO: is there a better way
        //Forced result since we use if let
        false
    }
}

impl<'a> cmp::Eq for Column<'a> {}

/// # TableRef
pub type ColumnList<'a> = Vec<Column<'a>>;
pub type TableList<'a> = HashMap<Table<'a>, ColumnList<'a>>;

#[derive(Debug)]
pub enum From<'a> {
    List(TableList<'a>),
    One(Table<'a>, ColumnList<'a>)
}

impl<'a> From<'a> {
    pub fn new_list() -> Self {
        From::List(HashMap::new())
    }

    pub fn only_one(table: &'a str) -> Self {
        From::One(Table::Name(table), From::columns_list())
    }

    pub fn columns_list() -> ColumnList<'a> {
        Vec::<Column<'a>>::new()
    }

    pub fn len(&self) -> usize {
        match self {
            From::List(map) => map.len(),
            From::One(_, _) => 1
        }
    }
}

impl<'a> Hash for From<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            From::List(map) => {
                for (table, columns) in map.iter() {
                    table.hash(state);
                    columns.hash(state);
                }
            },
            From::One(table, columns) => {
                table.hash(state);
                columns.hash(state);
            }
        };
    }
}
