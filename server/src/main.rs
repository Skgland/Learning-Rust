#![feature(box_syntax)]
#![feature(unsized_locals)]

pub extern crate chat_base as lib;
pub extern crate chat_network as network;

mod server;

fn main() {
    server::server_entry_point();
}
