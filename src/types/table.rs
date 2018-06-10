use std::{
    cmp,
    convert,
};

use super::Aliasable;
use ::*;

impl<'a> Aliasable<'a> for Table<'a> {
    /// Get Table identifier
    ///
    /// # Examples
    ///
    /// Using Alias
    /// ````
    /// use qal::types::*;
    /// let table = Table::Alias("my_table", "mt");
    /// assert_eq!(table.identifier(), "mt");
    /// ````
    ///
    /// Using Name
    /// ````
    /// use qal::types::*;
    /// let table = Table::Name("my_table");
    /// assert_eq!(table.identifier(), "my_table");
    /// ````
    fn identifier(&self) -> &'a str {
        match self {
            Table::Alias(_, alias) => alias,
            Table::Name(name) => name
        }
    }

    /// Get Table name
    ///
    /// # Examples
    ///
    /// Using Alias
    /// ````
    /// use qal::types::*;
    /// let table = Table::Alias("my_table", "mt");
    /// assert_eq!(table.name(), "my_table");
    /// ````
    ///
    /// Using Name
    /// ````
    /// use qal::types::*;
    /// let table = Table::Name("my_table");
    /// assert_eq!(table.name(), "my_table");
    /// ````
    fn name(&self) -> &'a str {
        match self {
            Table::Alias(name, _) => name,
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
