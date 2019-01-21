pub extern crate chat_base as lib;
pub extern crate chat_network as network;

mod client;

fn main() {
    client::client_entry_point();
}
