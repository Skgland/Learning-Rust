use std::string::*;
use std::sync::mpsc::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;

use super::addons::alias::*;

///
/// A type containing the state for a User on the Chat Server
///
pub struct UserIdentifier<A> {
    pub username: String,
    pub connections: Vec<Sender<Pass<A>>>,
}

impl<A> UserIdentifier<A> where  A:Read+Write{
    pub fn new(name: &str) -> UserIdentifier<A> {
        UserIdentifier{ username: name.to_string(),connections: Vec::new()}
    }

    //#[not(warn(unused))]
    pub fn rename (self,new_name: &str) -> Self {
        UserIdentifier{username:new_name.to_string(),connections: self.connections}
    }
}


pub struct Server { pub stream: TcpListener, pub user_map: UIDMap<TcpStream>, pub commands: Commands<Server> }

pub struct ServerConnection{pub stream: TcpStream,pub username:String}

pub struct Client { pub stream: TcpStream, pub commands: Commands<Client> }
