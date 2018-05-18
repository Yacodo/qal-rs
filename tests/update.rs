extern crate qal;

use qal::Connector;
// use qal::types::*;

use qal::hr::Hr;

#[test]
fn update_empty(){
    let c = Hr{};
    assert_eq!(
        c.update("my_table").to_string(),
"UPDATE
\tmy_table
SET
\t1=1"
    );
}
