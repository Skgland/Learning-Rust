#![feature(specialization)]

use std::env;

pub mod api;
pub mod commandline;

pub mod prelude {
    pub mod v1 {
        pub use crate::api::{alias::*, traits::{*,serialization::*}, structs::*};
        pub use crate::commandline::*;
    }
}

///
/// The function initially called by the chat server and client
/// Prints general system information to the console
///

pub fn common_entry() {
    println!("OS: {}", env::consts::OS)

}

