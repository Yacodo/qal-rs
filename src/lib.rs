pub mod select;

use std::fmt::Display;

pub trait Query: Display + ToString {
    fn new() -> Self;
}

pub struct Select {

}
