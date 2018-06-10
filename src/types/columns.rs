use super::*;

impl<'c> Columns<'c> {

    /// Create a new Columns list
    ///
    /// # Examples
    ///
    /// ````
    /// use qal::types::*;
    /// let columns = Columns::new();
    /// assert!(columns.len() == 1);
    /// assert!(columns.query().is_none());
    /// ````
    pub fn new() -> Columns<'c> {
        Columns {
            columns: vec![Column::All]
        }
    }

    ///TODO
    pub fn add<C: Into<Column<'c>>>(mut self, column: C) -> Self {
        let column = column.into();

        // If new column is all, replace current vec with 1 capacity
        if let Column::All = column {
            if self.len() > 1 {
                drop(&self.columns);
                self.columns = Vec::with_capacity(1);
            }
        }else{
            //if new column is not all && current only column is, replace with empty new vec
            if self.len() == 1 && self.columns[0].is_all() {
                drop(&self.columns);
                self.columns = Vec::new();
            }
        }

        self.columns.push(column);

        self
    }

    /// Check len of columns
    ///
    /// # Examples
    ///
    /// ````
    /// use qal::types::*;
    /// let columns = Columns::new_list();
    /// assert!(columns.len() == 1);
    /// ````
    pub fn len(&self) -> usize {
        self.columns.len()
    }
}
