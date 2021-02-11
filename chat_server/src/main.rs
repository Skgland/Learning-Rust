
pub use chat_base as lib;
pub use network_lib as network;

mod server;

fn main() {
    server::server_entry_point();
}
