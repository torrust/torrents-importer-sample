#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

pub mod action;
pub mod app;
pub mod config;
pub mod source;
pub mod statistics;
pub mod target;
pub mod ui;
pub mod utils;

#[tokio::main]
async fn main() {
    app::run().await;
}
