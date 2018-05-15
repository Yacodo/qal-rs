use std::fmt;
// use query::Query;

use super::{Select, Query};

impl Query for Select {
    /// Create a new select
    ///
    /// # Example
    ///
    /// ````
    /// use qal::*;
    /// let select = Select::new();
    /// ````
    fn new() -> Select {
        Select {}
    }
}


impl fmt::Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
