use crate::lib::prelude::v1 as lib;
use crate::lib::part;


use std::io::*;
use std::net::TcpStream;
use self::lib::Serializable;

pub struct ChannelPair<'a> {
    to_client: &'a mut lib::ReadWrite<'a>,
    to_server: &'a mut lib::ReadWrite<'a>,
}

impl<'a> ChannelPair<'a> {
    pub fn to_server(&mut self) -> &mut lib::ReadWrite<'a>{
        self.to_server
    }
    pub fn to_client(&mut self) -> &mut lib::ReadWrite<'a> {
        self.to_client
    }
}

pub struct ClientInitInput{
    pub commands: lib::Commands<lib::Client>
}
pub struct ServerInitInput {}

pub type ClientInitOutput = lib::Client;
pub type ServerInitOutput = lib::ServerConnection;

pub enum InitSide<S,C> {
    Server(S),
    Client(C),
}

impl<S,C> InitSide<S,C> {
    fn is_server(&self) -> bool {
        if let InitSide::Server(..) = self {
            true
        } else {
            false
        }
    }
    fn is_client(&self) -> bool {
        if let InitSide::Client(..) = self {
            true
        } else {
            false
        }
    }
}

pub enum Connection {
    Server(ServerInitOutput),
    Client(ClientInitOutput),
}

pub mod init {
    use super::*;


    const CURRENT_VERSION: u32 = 1;
    const USERNAME_QUESTION: &str = "Your name, please: ";

    pub fn client_connection(client:ClientInitInput,stream:TcpStream) -> Result<ClientInitOutput>{
        match connection_init(InitSide::Client(client),stream) {
            Err(e) => {
                Err(e)
            }
            Ok(InitSide::Client(c))=>{
                Ok(c)
            }
            _ => {
                Err(Error::new(ErrorKind::InvalidData,"Got ServerInitOutput while initializing Client"))
            }
        }
    }

    pub fn server_connection(server: ServerInitInput, stream: TcpStream) -> Result<ServerInitOutput> {
        match connection_init(InitSide::Server(server), stream) {
            Err(e) => {
                Err(e)
            }
            Ok(InitSide::Server(s)) => {
                Ok(s)
            }
            _ => {
                Err(Error::new(ErrorKind::InvalidData, "Got ClientInitOutput while initializing Server"))
            }
        }
    }

    fn connection_init(side: InitSide<ServerInitInput,ClientInitInput>, mut stream: TcpStream) -> Result<InitSide<ServerInitOutput, ClientInitOutput>> {
        let stream_copy = &mut part!( stream in (Ok(stream)  =  stream.try_clone()) else ?);

        let mut read = lib::ReadWrite::READ { reader: &mut stream };
        let mut write = lib::ReadWrite::WRITE { writer: stream_copy };

        let channel = match side {
            InitSide::Server(..) => {
                println!("Connection received");
                ChannelPair { to_client: &mut write, to_server: &mut read }
            }
            InitSide::Client(..) => {
                println!("Connection Successful");
                ChannelPair { to_client: &mut read, to_server: &mut write }
            }
        };

        let (mut version_client,mut version_server) = if side.is_client() {
            (CURRENT_VERSION,0)
        } else{
            (0,CURRENT_VERSION)
        };

        version_client.serialize(channel.to_server)?;
        version_server.serialize(channel.to_client)?;

        if version_server != version_client {
            return Err(Error::new(ErrorKind::ConnectionAborted,"Version mismatch between Server and Client"))
        }

        let mut username = if side.is_client() {
            println!("Sending Username ...");
            part!(name in (Ok(name) = lib::ask(USERNAME_QUESTION)) else retry)
        } else {
            println!("Expecting Username ...");
            String::new()
        };


        Serializable::serialize(&mut username,channel.to_server)?;


        let debug = {
            #[cfg(test)]
            {true}

            #[cfg(not(test))]
            {false}
        };

        if debug {
            let mut test = lib::Test { name: "Wrong".to_string(), age: -42 };

            if side.is_client() {
                test.name = "Correct".to_string();
                test.age = 42;
            }

            test.serialize(channel.to_server)?;

            if side.is_server() {
                println!("Test{{name: {}, age: {}}}", test.name, test.age);
            }
        }

        stream.flush().unwrap();

        if side.is_client() {
            Ok(InitSide::Client(lib::Client { stream, commands: Vec::new() }))
        } else {
            println!("New User {}", &username);
            Ok(InitSide::Server(lib::ServerConnection { stream, username }))
        }
    }
}
