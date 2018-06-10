extern crate qal;

use qal::Connector;
// use qal::types::*;

use qal::hr::Hr;

#[test]
fn delete_all(){
    let hr = Hr{};
    assert_eq!(
        hr.delete("my_table").to_string(),
"DELETE FROM
\tmy_table"
    );
}
