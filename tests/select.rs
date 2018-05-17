extern crate qal;

use qal::Connector;
use qal::types::*;

use qal::hr::Hr;

#[test]
fn select_all(){
    let c = Hr{};

    assert_eq!(
        c.select("my_table").to_string(),
        "SELECT\r\n\
            \t*\r\n\
        FROM\r\n\
            \t\"my_table\""
    );
}

#[test]
fn select_from_two_tables(){
    let c = Hr{};
    let mut select = c.select("xyz");
    select.from("abc");

    assert_eq!(
        select.to_string(),
        "SELECT\r\n\
            \t*\r\n\
        FROM\r\n\
            \t\"xyz\", \"abc\""
    );


    let mut select = c.select("abc");
    select.from("xyz");

    assert_eq!(
        select.to_string(),
        "SELECT\r\n\
            \t*\r\n\
        FROM\r\n\
            \t\"abc\", \"xyz\""
    );
}

#[test]
fn select_from_two_tables_one_alias(){
    let c = Hr{};
    let mut select = c.select("xyz");

    select.from(Table::Alias("abc", "t2"));

    assert_eq!(
        select.to_string(),
        "SELECT\r\n\
            \t*\r\n\
        FROM\r\n\
            \t\"xyz\", \"abc\" AS t2"
    );

    let mut select = c.select(Table::Alias("abc", "t1"));

    select.from("xyz");

    assert_eq!(
        select.to_string(),
        "SELECT\r\n\
            \t*\r\n\
        FROM\r\n\
            \t\"abc\" AS t1, \"xyz\""
    );
}
