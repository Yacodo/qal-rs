extern crate qal;

use qal::Connector;
use qal::types::*;

//standalone connector
mod connector;
use connector::Test;

#[test]
fn empty(){
    let c = Test{};
    assert_eq!(c.update("my_table").to_string(), "UPDATE my_table SET 1=1");
}

#[test]
fn from_one(){
    let c = Test{};
    let select = c.delete("");

    match select.tables() {
        From::List(_) => panic!(
            "Update queries should have TableRef::One() instead of TableRef::List()"
        ),
        _ => return
    }
}
