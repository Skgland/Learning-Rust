#![feature(specialization)]

extern crate macros;
use std::env;

pub mod api;
pub mod commandline;

pub mod prelude {
    pub mod v1 {
        pub use crate::api::{alias::*, traits::{*,serialization::*}, structs::*};
        pub use crate::commandline::*;
    }
}
pub fn common_entry() {
    println!("OS: {}", env::consts::OS)

}

