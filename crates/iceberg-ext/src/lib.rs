#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub,
    clippy::pedantic
)]
#![forbid(unsafe_code)]

pub mod catalog;
pub mod spec;
pub mod validation;

pub use iceberg::{
    NamespaceIdent, TableCommit, TableCreation, TableIdent, TableRequirement, TableUpdate,
};

pub use iceberg;
pub use iceberg_datafusion;

#[macro_use]
extern crate serde_derive;
