pub mod table;
pub mod column;

pub mod from;
pub mod columns;

use std::{
    collections::HashMap
};

pub type Tables<'t> = Vec<Table<'t>>;
pub type TableColumns<'tc> = HashMap<
    &'tc Table<'tc>,
    Columns<'tc>
>;

#[derive(Debug, PartialEq)]
pub enum QueryType {
    Select,
    Update,
    Delete,
}

#[derive(Debug, PartialEq)]
pub enum JoinType {
    Join,
    LeftJoin,
    RightJoin,
    //TODO MOAR
}

pub trait Aliasable<'a> {
    /// Get the identifier for SQL request
    /// Should be alias if provided, else name
    fn identifier(&self) -> &'a str;
    /// Should always be the name
    fn name(&self) -> &'a str;
}

#[derive(Debug, Hash)]
pub enum Column<'a> {
    /// Alias - Reads: Alias {1} by {2}
    Alias(&'a str, &'a str),
    Name(&'a str),
    All
}

pub struct Columns<'c> {
    pub columns: Vec<Column<'c>>
}

#[derive(Debug, Hash)]
pub enum Table<'a> {
    /// Alias - Reads: Alias {1} by {2}
    Alias(&'a str, &'a str),
    /// Table name
    Name(&'a str)
}

#[derive(Debug)]
pub enum From<'a> {
    List(Tables<'a>),
    One(Table<'a>)
}
