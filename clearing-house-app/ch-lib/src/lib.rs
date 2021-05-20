#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate error_chain;
extern crate fern;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
pub mod db;
pub mod model;