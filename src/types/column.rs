use std::{
    cmp,
    convert,
};

use super::*;

impl<'c> Column<'c> {
    pub fn is_all(&self) -> bool {
        match self {
            Column::All => true,
            _ => false
        }
    }

    pub fn is_alias(&self) -> bool {
        match self {
            Column::Alias(_, _) => true,
            _ => false
        }
    }

    pub fn is_name(&self) -> bool {
        match self {
            Column::Name(_) => true,
            _ => false
        }
    }
}

impl<'c> Aliasable<'c> for Column<'c> {
    /// Get Column alias
    ///
    /// # Examples
    ///
    /// Using Alias
    /// ````
    /// use qal::types::*;
    /// let column = Column::Alias("my_column", "mc");
    /// assert_eq!(column.identifier(), "mc");
    /// ````
    ///
    /// Using Name
    /// ````
    /// use qal::types::*;
    /// let column = Column::Name("my_column");
    /// assert_eq!(column.identifier(), "my_column");
    /// ````
    fn identifier(&self) -> &'c str {
        match self {
            Column::Alias(_, alias) => alias,
            Column::Name(name) => name,
            Column::All => "*"
        }
    }

    /// Get Column name
    ///
    /// # Examples
    ///
    /// Using Alias
    /// ````
    /// use qal::types::*;
    /// let column = Column::Alias("my_column", "mc");
    /// assert_eq!(column.name(), "my_column");
    /// ````
    ///
    /// Using Name
    /// ````
    /// use qal::types::*;
    /// let column = Column::Name("my_column");
    /// assert_eq!(column.name(), "my_column");
    /// ````
    fn name(&self) -> &'c str {
        match self {
            Column::Alias(name, _) => name,
            Column::Name(name) => name,
            Column::All => "*"
        }
    }
}

impl<'c> cmp::PartialEq for Column<'c> {
    fn eq(&self, other: &Column<'c>) -> bool {
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

impl<'c> cmp::Eq for Column<'c> {}

impl<'c> convert::From<&'static str> for Column<'c> {
    fn from(table: &'static str) -> Self {
        Column::Name(table)
    }
}
