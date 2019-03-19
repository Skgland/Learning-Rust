#![feature(box_syntax)]
#![feature(unsized_locals)]

pub use chat_base as lib;
pub use chat_network as network;

mod server;

fn main() {
    server::server_entry_point();
}
