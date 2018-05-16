extern crate qal;

use qal::Connector;
use qal::types::*;

//standalone connector
mod connector;
use connector::Test;

#[test]
fn select_all(){
    let c = Test{};

    assert_eq!(c.select("my_table").to_string(), "SELECT * FROM my_table");
}

#[test]
fn from_list(){
    let c = Test{};
    let select = c.select("");

    match select.tables() {
        From::One(_, _) => panic!(
            "Select query should have TableRef::List() instead of TableRef::One()"
        ),
        _ => return
    }
}
