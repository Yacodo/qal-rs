extern crate qal;

use qal::Connector;

//standalone connector
mod connector;
use connector::Test;
use qal::types::*;

#[test]
fn delete_all(){
    let c = Test{};
    assert_eq!(c.delete("my_table").to_string(), "DELETE FROM my_table");
}

#[test]
fn delete_from_one(){
    let c = Test{};
    let select = c.delete("");

    match select.tables() {
        From::List(_) => panic!(
            "Delete queries should have TableRef::One() instead of TableRef::List()"
        ),
        _ => return
    }
}
