#[macro_use]
extern crate exonum;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate bodyparser;

extern crate iron;
extern crate router;

#[cfg(test)]
extern crate iron_test;
#[cfg(test)]
extern crate mime;

mod service;
pub use service::{DigitalContractService};

pub mod model;
pub mod api;

pub mod transactions;

pub const DC_SERVICE_ID: u16 = 128;
pub const TX_CREATE_CONTRACTOR_ID: u16 = 1;