pub use chat_base as lib;
pub use network_lib as network;

mod client;

fn main() {
    client::client_entry_point();
}
