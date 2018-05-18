use std::{
    cmp,
    convert,
    //iter, Consider
    rc::Rc,
    // hash::{Hash, Hasher}, useless now
    collections::HashMap
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

pub trait Aliasable<'a> {
    fn identifier(&self) -> &'a str;
}

/// # Table

#[derive(Debug, Hash)]
pub enum Table<'a> {
    Alias(&'a str, &'a str),
    Name(&'a str)
}

impl<'a> Aliasable<'a> for Table<'a> {
    fn identifier(&self) -> &'a str {
        match self {
            Table::Alias(_, alias) => alias,
            Table::Name(name) => name
        }
    }
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

impl<'a> convert::Into<Table<'a>> for &'static str {
    fn into(self) -> Table<'a> {
        Table::Name(self)
    }
}

/// # From

pub type TableList<'a> = Vec<Rc<Table<'a>>>;

#[derive(Debug)]
pub enum From<'a> {
    List(TableList<'a>),
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

    pub fn get_list(&'a mut self) -> Result<&'a mut TableList<'a>, bool> {
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

/// # Column

#[derive(Debug, Hash)]
pub enum Column<'a> {
    Alias(&'a str, &'a str),
    Name(&'a str)
}

impl<'a> Aliasable<'a> for Column<'a> {
    fn identifier(&self) -> &'a str {
        match self {
            Column::Alias(_, alias) => alias,
            Column::Name(name) => name
        }
    }
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

        false
    }
}

impl<'a> cmp::Eq for Column<'a> {}

impl<'a> convert::From<&'static str> for Column<'a> {
    fn from(table: &'static str) -> Self {
        Column::Name(table)
    }
}

/// # ColumnList

pub type ColumnList<'cl> = Vec<Column<'cl>>;
pub type TableColumns<'tc> = HashMap<Rc<Table<'tc>>, Columns<'tc>>;

pub enum Columns<'c> {
    List(ColumnList<'c>),
    All
}

impl<'c> Columns<'c> {
    /// Create a new Columns list
    ///
    /// ````
    /// use qal::types::Columns;
    /// let columns = Columns::new_list();
    /// assert!(columns.is_list());
    /// assert!(columns.len() == 0);
    /// ````
    pub fn new_list() -> Self {
        Columns::List(Vec::new())
    }

    /// Check if a Columns type is a list
    ///
    /// ````
    /// use qal::types::Columns;
    /// let columns = Columns::new_list();
    /// assert!(columns.is_list());
    /// ````
    pub fn is_list(&self) -> bool{
        if let Columns::List(_) = self {
            return true
        }

        false
    }

    /// Check if a Columns type is All
    ///
    /// ````
    /// use qal::types::Columns;
    /// let columns = Columns::All;
    /// assert!(columns.is_all());
    /// ````
    pub fn is_all(&self) -> bool{
        if let Columns::All = self {
            return true
        }

        false
    }

    /// Check len of columns
    ///
    /// ````
    /// use qal::types::Columns;
    /// let columns = Columns::new_list();
    /// assert!(columns.len() == 0);
    /// ````
    ///
    /// In case of `Columns::All` it will be of len "1" (-> translate as one column)
    ///
    /// ````
    /// use qal::types::Columns;
    /// let columns = Columns::All;
    /// assert!(columns.len() == 1);
    /// ````
    pub fn len(&self) -> usize {
        match self {
            Columns::List(columns) => columns.len(),
            _ => 1
        }
    }
}
