extern crate qal;

use qal::Connector;
use qal::types::*;

//standalone connector
mod connector;
use connector::Test;

#[test]
fn select_all(){
    let c = Test{};

    assert_eq!(
        c.select("my_table").to_string(),
        "SELECT * FROM \"my_table\""
    );
}

#[test]
fn select_from_list(){
    let c = Test{};
    let select = c.select("");

    match select.tables() {
        From::One(_) => panic!(
            "Select query should have TableRef::List() instead of TableRef::One()"
        ),
        _ => return
    }
}

#[test]
fn select_from_two_tables(){
    let c = Test{};
    let mut select = c.select("xyz");
    select.from("abc");

    assert_eq!(
        select.to_string(),
        "SELECT * FROM \"xyz\", \"abc\""
    );


    let mut select = c.select("abc");
    select.from("xyz");

    assert_eq!(
        select.to_string(),
        "SELECT * FROM \"abc\", \"xyz\""
    );
}

#[test]
fn select_from_two_tables_one_alias(){
    let c = Test{};
    let mut select = c.select("xyz");

    select.from(Table::Alias("abc", "t2"));

    assert_eq!(
        select.to_string(),
        "SELECT * FROM \"xyz\", \"abc\" AS t2"
    );

    let mut select = c.select(Table::Alias("abc", "t1"));

    select.from("xyz");

    assert_eq!(
        select.to_string(),
        "SELECT * FROM \"abc\" AS t1, \"xyz\""
    );
}
