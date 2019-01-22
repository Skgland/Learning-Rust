pub use init::*;
use lib::prelude::v1::*;

use lib::api::structs::Client as CL;
use std::io::*;
use std::net::TcpStream;


pub struct ChannelPair<'a> {
    to_client: &'a mut ReadWrite<'a>,
    to_server: &'a mut ReadWrite<'a>,
}

pub enum Side {
    Server(()),
    Client(Commands<CL>),
}

pub enum Connection {
    Server(ServerConnection),
    Client(Client),
}

pub mod init {
    use super::*;
    use lib::part;

    pub use super::Side;

    type A = Side;
    type B = Side;
    type C = Connection;
    type D = Connection;

    const USERNAME_QUESTION: &str = "Your name, please: ";

    macro_rules! choose {
        (server $client:tt $server:tt) => {
            $server
        };
        (client $client:tt $server:tt) => {
            $server
        };
    }

    macro_rules! dup {
        () => {
            dup!(server connection_init_server C);
            dup!(client connection_init_client D);
        };
        ($side:tt $name:ident $tp:ty) => {
            pub fn $name(side: choose!($side  A B), mut stream: TcpStream) -> Result<$tp> {
                let stream_copy = &mut part!( stream in? Ok(stream)  =  stream.try_clone());

                let mut read = ReadWrite::READ { reader: &mut stream };
                let mut write = ReadWrite::WRITE { writer: stream_copy };

                let to_client;
                let to_server;


                choose!($side
                    {
                        to_client = &mut read;
                        to_server = &mut write;
                        println!("Connection Successful");
                    }
                    {
                        to_server = &mut read;
                        to_client = &mut write;
                        println!("Connection received")
                    }
                );


                let mut username;

                choose!($side
                    {

                    username = part!(name try until Ok(name) = ask(USERNAME_QUESTION));
                    println!("Sending Username ...");
                    }
                    {

                    username = String::new();
                    println!("Expecting Username ...");
                    }
                );

                username.serialize(to_server)?;

                if false {
                    let mut test = Test { name: "Wrong".to_string(), age: -42 };
                    choose!($side
                        {
                            test.name = "Correct".to_string();
                            test.age = 42;
                        }
                        {

                        }
                    );

                    test.serialize(to_server)?;

                    choose!($side
                        {

                        }
                        {
                            println!("Test{{name: {}, age: {}}}", test.name, test.age);
                        }
                    );

                }

                stream.flush().unwrap();

                 choose!($side
                    {
                        use lib::api::structs;
                        Ok(Connection::Client(structs::Client { stream, commands: Vec::new() }))

                    }
                    {
                        println!("New User {}", &username);
                        Ok(Connection::Server(ServerConnection { stream, username }))

                    }
                )
            }
        };
    }

    dup!();

}
