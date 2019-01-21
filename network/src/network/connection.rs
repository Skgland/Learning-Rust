pub use init::*;
use lib::prelude::v1::*;

use lib::api::structs::Client as CL;

pub enum Side {
    Server(()),
    Client(Commands<CL>),
}

pub enum Connection {
    Server(ServerConnection),
    Client(Client),
}


pub mod init {
    use lib::prelude::v1::*;
    use lib::part;
    use crate::network::connection::Side;
    pub use crate::network::connection::Side::*;
    use std::io::Result;
    use crate::network::connection::Connection;
    use std::net::TcpStream;
    use std::io::Write;

    const USERNAME_QUESTION: &str = "Your name, please: ";

    pub fn connection_init(side: Side, mut stream: TcpStream) -> Result<Connection> {

        let stream_copy = &mut part!( stream in? Ok(stream)  =  stream.try_clone());

        let mut read = ReadWrite::READ { reader: &mut stream };
        let mut write = ReadWrite::WRITE { writer: stream_copy };

        let to_client;
        let to_server;

        {
            match side {
                Server(..) => {
                    to_server = &mut read;
                    to_client = &mut write;
                    println!("Connection received")
                }
                Client(..) => {
                    to_client = &mut read;
                    to_server = &mut write;
                    println!("Connection Successful");
                }
            }
        }

        let mut username = String::new();




        if let Client(..) = side{
            username = part!(name try until Ok(name) = ask(USERNAME_QUESTION));
            println!("Sending Username ...");
        }else{
            println!("Expecting Username ...");
        }

        username.serialize( to_server)?;

        if false {
            let mut test = Test { name: "Wrong".to_string(), age: -42 };

            if let Client(..) = side {
                test.name = "Correct".to_string();
                test.age = 42;
            }

            test.serialize(to_server);

            if let Server(..) = side {
                println!("Test{{name: {}, age: {}}}", test.name, test.age);
            }


        }

        stream.flush().unwrap();

        if let Server(_) = side {
            println!("New User {}", &username);
            Ok(Connection::Server(ServerConnection{stream,username}))
        }else{
            use  lib::api::structs;
            Ok(Connection::Client(structs::Client{stream,commands:Vec::new()}))
        }
    }
}
