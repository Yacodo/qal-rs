use std::{
    cmp,
    convert,
    //iter,
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

// impl<'a> convert::From<&'static str> for Table<'a> {
//     fn from(table: &'static str) -> Self {
//         Table::Name(table)
//     }
// }

impl<'a> convert::Into<Table<'a>> for &'static str {
    fn into(self) -> Table<'a> {
        Table::Name(self)
    }
}

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

impl<'a> convert::From<&'static str> for Column<'a> {
    fn from(table: &'static str) -> Self {
        Column::Name(table)
    }
}

/// # TableRef
pub type ColumnList<'a> = Vec<Column<'a>>;
pub type TableList<'a> = HashMap<Table<'a>, ColumnList<'a>>;

#[derive(Debug)]
pub enum From<'a> {
    List(Vec<Table<'a>>),
    One(Table<'a>)
}

impl<'a> From<'a> {
    pub fn new_list() -> Self {
        From::List(Vec::new())
    }

    pub fn only_one(table: Table<'a>) -> Self {
        From::One(table)
    }

    // pub fn columns_list() -> ColumnList<'a> {
    //     Vec::<Column<'a>>::new()
    // }

    pub fn len(&self) -> usize {
        match self {
            From::List(tables) => tables.len(),
            From::One(_) => 1
        }
    }

    pub fn get_list(&'a mut self) -> Result<&'a mut Vec<Table<'a>>, bool> {
        match self {
            From::List(ref mut tables) => Ok(tables),
            _ => return Err(false)
        }
    }

    pub fn get_table(&'a self) -> Result<&Table<'a>, bool> {
        match self {
            From::One(ref table) => Ok(table),
            _ => return Err(false)
        }
    }
}

impl<'a> Hash for From<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            From::List(tables) => {
                for table in tables {
                    table.hash(state);
                }
            },
            From::One(table) => {
                table.hash(state);
            }
        };
    }
}
