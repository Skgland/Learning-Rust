pub use chat_base as lib;
pub use chat_network as network;

mod client;

fn main() {
    client::client_entry_point();
}
