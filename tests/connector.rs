extern crate qal;
use qal::Connector;
use qal::formatters::*;

pub struct Test {}
impl<'q> Connector<'q> for Test {
    fn print_select(&self, columns: &str, tables: &str) -> String {
        format!(
            "SELECT {} FROM {}",
            columns,
            tables,
        )
    }

    fn print_update(&self, table: &str, values: &str) -> String {
        format!("UPDATE {} SET {}", table, values)
    }

    fn print_delete(&self, table: &str) -> String {
        format!("DELETE FROM {}", table)
    }

}

impl<'a> FormatColumn<'a> for Test {}
impl<'a> FormatTable<'a> for Test {}
