extern crate qal;

use qal::Connector;

//standalone connector
mod connector;
use connector::Test;

#[test]
fn new_select(){
    let c = Test{};
    let select = c.select("my_table");
    assert!(select.is_select());
}

#[test]
fn new_update(){
    let c = Test{};
    let select = c.update("my_table");
    assert!(select.is_update());
}

#[test]
fn new_delete(){
    let c = Test{};
    let select = c.delete("my_table");
    assert!(select.is_delete());
}

#[test]
fn is_hr(){
    let c = Test{};
    assert!(c.hr());
}

#[test]
fn toggle_hr(){
    let c = Test{};
    assert!(c.toggle_hr().is_err());
}
