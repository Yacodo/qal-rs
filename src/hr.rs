use ::{
    Connector,
    formatters::*
};

/// # Human Readable SQL
///
/// Primarily use for QAL Testing
///
/// It is not following any SQL standard in particular
///
/// It can be used for debugging your own sql queries.
///
/// ## Testing
///
/// - Unit tests of this represent specific behavior
/// - Integration tests check for use of implementation in general
pub struct Hr {}

impl<'q> Connector<'q> for Hr {
    fn print_select(&self, columns: &str, tables: &str) -> String {
        format!(
"SELECT
\t{}
FROM
\t{}",
            columns,
            tables,
        )
    }

    fn print_update(&self, table: &str, values: &str) -> String {
        format!(
"UPDATE
\t{}
SET
\t{}",
            table,
            values
        )
    }

    fn print_delete(&self, table: &str) -> String {
        format!(
"DELETE FROM
\t{}",
            table
        )
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
                "columns_list",
                "table, list"
            ),
"SELECT
\tcolumns_list
FROM
\ttable, list"
        );
    }

    #[test]
    fn hr_update(){
        let hr = Hr{};
        assert_eq!(
            Hr::print_update(
                &hr,
                "table AS t",
                "foo='bar',\n\t1=1"
            ),
"UPDATE
\ttable AS t
SET
\tfoo='bar',
\t1=1"
        );
    }

    #[test]
    fn hr_delete(){
        let hr = Hr{};
        assert_eq!(
            Hr::print_delete(
                &hr,
                "deleting"
            ),
"DELETE FROM
\tdeleting"
        );
    }
}
