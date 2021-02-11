use network_lib as network;
use network::protocol::*;
use network::serialization::*;
use std::io::ErrorKind;
use std::io::Error;

pub mod addons;
pub mod structs;

pub enum Side {
    Server,
    Client,
}

pub struct InitState {
    side: Side,
    their_version: u32,
    our_version: u32,
    pub username: String,
    our_test: bool,
    their_test: bool,
    test_data: Option<Test>,
}

impl InitState {
    pub fn new(side:Side) -> Self {
        InitState {
            side,
            their_version: crate::UNKNOWN_VERSION,
            our_version: crate::CURRENT_VERSION,
            username: String::new(),
            our_test: false,
            their_test: false,
            test_data: None,
        }
    }
}

pub enum InitStateAccessor {
    OurVersion,
    TheirVersion,
    Username,
    OurTest,
    TheirTest,
    TestData,
}

impl StateAccess<InitState> for InitStateAccessor {
    fn get_access<'b>(&self, state: &'b mut InitState) -> std::io::Result<&'b mut dyn Serializable> {
        use InitStateAccessor::*;
        let result: &mut dyn Serializable = match self {
            OurVersion => &mut state.our_version,
            TheirVersion => &mut state.their_version,
            Username => &mut state.username,
            OurTest => &mut state.our_test,
            TheirTest => &mut state.their_test,
            TestData => {
                match &mut state.test_data {
                    Some(data) => data,
                    None => return Err(Error::new(ErrorKind::NotFound, "TestData was None"))
                }
            }
        };
        Ok(result)
    }
}

use Protocol::*;
use Destination::*;

pub fn init_protocol() -> Protocol<InitState, InitStateAccessor> {
    Chain(vec! {
        Exchange(InitStateAccessor::OurVersion,InitStateAccessor::TheirVersion),
        Conditional(Box::new(|state: &InitState| state.their_version != state.our_version),
                    Box::new(Update(Box::new(|state:&mut InitState| Err(Error::new(ErrorKind::ConnectionAborted, format!("Version mismatch between Server({}) and Client({})", state.our_version, state.their_version)))))),
                    Box::new(Chain(vec! {
                Update(Box::new(pre_username_transfer)),
                Exchange(InitStateAccessor::OurTest,InitStateAccessor::TheirTest),
                Conditional(
                    Box::new(|state: &InitState| !state.their_test || !state.our_test),
                    Box::new(Update(Box::new(|_| {
                        println!("No Test");
                        Ok(())
                    }))),
                    Box::new(Chain(vec![
                        Transfer(ToServer, InitStateAccessor::TestData),
                        Update(Box::new(post_testdata_transfer)),
                    ])),
                ),
            })),
        )
    })
}

pub const UNKNOWN_VERSION: u32 = 0;
pub const CURRENT_VERSION: u32 = 1;
const USERNAME_QUESTION: &str = "Your name, please: ";

fn pre_username_transfer(state: &mut InitState) -> std::io::Result<()> {
    use base_lib::part;

    if let Side::Client = &state.side {
        state.username = part!(name in (Ok(name) = base_lib::commandline::ask(USERNAME_QUESTION)) else retry)
    }

    Ok(())
}

fn post_testdata_transfer(state: &mut InitState) -> std::io::Result<()> {
    if let (Side::Server, Some(test)) = (&state.side, &state.test_data) {
        println!("Test{{name: {}, age: {}}}", test.name, test.age);
    }

    Ok(())
}
