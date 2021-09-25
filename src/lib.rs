#![warn(clippy::disallowed_method)]

#[macro_use]
extern crate shadow_rs;

shadow!(shadow);

// Lib is present to allow for benchmarking
pub mod bug_report;
pub mod config;
pub mod configs;
pub mod configure;
pub mod context;
pub mod formatter;
pub mod init;
pub mod logger;
pub mod module;
mod modules;
pub mod print;
mod segment;
mod utils;

#[cfg(test)]
mod test;
