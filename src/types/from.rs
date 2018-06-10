use super::*;
// use ::*;

impl<'a> From<'a> {
    /// Create a new From list
    ///
    /// ````
    /// use qal::types::*;
    /// let columns = From::new_list();
    /// assert!(columns.is_list());
    /// assert!(columns.len() == 0);
    /// ````
    pub fn new_list() -> Self {
        From::List(Vec::new())
    }

    /// Check if a From type is a list
    ///
    /// ````
    /// use qal::types::*;
    /// let columns = From::new_list();
    /// assert!(columns.is_list());
    /// ````
    pub fn is_list(&self) -> bool{
        if let From::List(_) = self {
            return true
        }

        false
    }

    /// Create a new From list
    ///
    /// ````
    /// use qal::types::*;
    /// let columns = From::one("my_table");
    /// assert!(columns.is_one());
    /// assert!(columns.len() == 1);
    /// ````
    pub fn one<T: Into<Table<'a>>>(table: T) -> Self {
        From::One(table.into())
    }

    /// Check if a From type is only one table
    ///
    /// ````
    /// use qal::types::*;
    /// let columns = From::one("my_table");
    /// assert!(columns.is_one());
    /// ````
    pub fn is_one(&self) -> bool{
        if let From::One(_) = self {
            return true
        }

        false
    }

    /// Check len of From
    ///
    /// #Examples
    ///
    /// ````
    /// use qal::types::*;
    /// let columns = From::new_list();
    /// assert!(columns.len() == 0);
    /// ````
    ///
    /// In case of `From::One(_)` it will be of len "1"
    ///
    /// ````
    /// use qal::types::*;
    /// let columns = From::one("my_table");
    /// assert!(columns.len() == 1);
    /// ````
    pub fn len(&self) -> usize {
        match self {
            From::List(tables) => tables.len(),
            From::One(_) => 1
        }
    }


    /// Get a Result containing the mutable TableList out of From
    ///
    /// #Examples
    ///
    /// ````
    /// use qal::types::*;
    /// let mut from = From::new_list();
    /// //--populate list (optional)
    /// assert!(from.get_list().is_ok());
    /// ````
    pub fn get_list(&'a mut self) -> Result<&'a mut Tables<'a>, bool> {
        match self {
            From::List(ref mut tables) => Ok(tables),
            _ => return Err(false)
        }
    }

    /// Get a Result containing the selected table
    ///
    /// #Examples
    ///
    /// ````
    /// use qal::types::*;
    /// let from = From::one("my_table");
    /// assert!(from.get_table().is_ok());
    /// assert_eq!(from.get_table().unwrap().identifier(), "my_table");
    /// ````
    pub fn get_table(&'a self) -> Result<&Table<'a>, bool> {
        match self {
            From::One(ref table) => Ok(table),
            _ => return Err(false)
        }
    }
}
