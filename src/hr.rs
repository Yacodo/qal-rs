use ::{
    Connector,
    formatters::*
};

/// # Human Readable SQL
///
/// Primarily use for QAL Testing
/// It is not following any SQL standard in particular
///
/// It can be used for debugging your own sql queries.
pub struct Hr {}

impl<'q> Connector<'q> for Hr {
    fn print_select(&self, columns: String, tables: String) -> String {
        format!(
            "SELECT\r\n\t{}\r\nFROM\r\n\t{}",
            columns,
            tables,
        )
    }

    fn print_update(&self, table: String, values: String) -> String {
        format!("UPDATE\r\n\t{}\r\nSET\r\n\t{}", table, values)
    }

    fn print_delete(&self, table: String) -> String {
        format!("DELETE FROM\r\n\t{}", table)
    }
}

impl<'a> FormatColumn<'a> for Hr {}
impl<'a> FormatTable<'a> for Hr {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hr_select(){
        let hr = Hr{};
        assert_eq!(
            Hr::print_select(
                &hr,
                "columns_list".to_owned(),
                "table, list".to_owned()
            ),
            "SELECT\r\n\
                \tcolumns_list\r\n\
            FROM\r\n\
                \ttable, list"
        );
    }

    #[test]
    fn hr_update(){
        let hr = Hr{};
        assert_eq!(
            Hr::print_update(
                &hr,
                "table AS t".to_owned(),
                "foo='bar'\r\n1=1".to_owned()
            ),
            "UPDATE\r\n\
                \ttable AS t\r\n\
            SET\r\n\
                \tfoo='bar'\r\n1=1"
        );
    }

    #[test]
    fn hr_delete(){
        let hr = Hr{};
        assert_eq!(
            Hr::print_delete(
                &hr,
                "deleting".to_owned()
            ),
            "DELETE FROM\r\n\
                \tdeleting"
        );
    }
}
