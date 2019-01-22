use std::prelude::v1::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::net::ToSocketAddrs;
use std::sync::Mutex;
use std::io::Result;

use crate::lib::prelude::v1::*;
use std::io::Write;
use std::io::Read;
use std::sync::mpsc::Sender;
use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;

pub fn server_entry_point() {
    crate::lib::common_entry();

    if let Some(server) = server_init("127.0.0.1:8888") {
        server_loop(server);
    }
}

pub fn server_init<A: ToSocketAddrs>(sock: A) -> Option<Server> {
    let user_map: UIDMap<TcpStream> = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind(sock).unwrap();

    Some(Server { stream: listener, user_map, commands: Vec::new() })
}

pub fn server_loop(Server { stream: listener, user_map, .. }: Server) {
    println!("Starting to wait for connections!");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let um_clone = user_map.clone();
                thread::spawn(move || handle_connection(stream, um_clone));
            }
            Err(e) => { println!("{}", e) }
        }
    }
    println!("The end!");
}

fn handle_connection(stream: TcpStream, user_map: UIDMap<TcpStream>) -> Result<()> {
    if let Ok((tx,rx, username,stream)) = connection_init(stream, &user_map) {
        let stream_copy = stream.try_clone()?;

        thread::spawn(move || connection_send_loop(stream_copy, rx, &user_map,username));
        connection_receive_loop(stream,tx);
    }
    Ok(())
}

type InitReturn = Result<(Sender<Pass<TcpStream>>, ServerReceiver<TcpStream>, String, TcpStream)>;

fn connection_init(stream: TcpStream, user_map: &UIDMap<TcpStream>) -> InitReturn {

    use crate::network::connection;
    use  connection::init;

    if let Ok(connection::Connection::Server(ServerConnection{stream,username})) =  init::connection_init(init::Server(()),stream){
        let mut uid = UserIdentifier::new(&username);

        let (tx, rx): (_, ServerReceiver<TcpStream>) = channel();

        uid.connections.push(tx.clone());
        {
            let mut map = user_map.lock().unwrap();

            map.insert(username.clone(), Vec::new());
        }

        Ok((tx, rx, username,stream))
    } else {
        Err(Error::new(ErrorKind::InvalidData,"Expected Server got something else"))
    }

}

fn connection_send_loop<R>(mut stream: R, rx: ServerReceiver<R>, user_map: &UIDMap<R>, mut username: String) where R: Read + Write {
    loop {
        match rx.recv() {
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
            Ok(CloseConnection) => {
                let mut  map = user_map.lock().unwrap();
                map.remove(&username);
                break;
            }
            Ok(Function(tmp)) => {
                tmp(&mut stream,user_map,&mut username);
            }
        }
    }
}

#[allow(clippy::collapsible_if)]
fn connection_receive_loop<R>(mut stream: R, tx: Sender<Pass<R>>) where R: Read + Write {
    let mut line: String = String::new();

    loop {
        line.serialize(&mut ReadWrite::READ { reader: &mut stream }).expect("Error");

        if line.starts_with('!') {
            if line.starts_with("!rename") {
                let new_name:String = line.trim_start_matches("!rename ").to_string();

                let tmp_rename
                = Box::new(move |a:&mut R,b:&UIDMap<R>,c :&mut String| rename_user(new_name, a, b, c));

                if let Err(e) = tx.send(Pass::Function(tmp_rename)){
                    eprintln!("{}",e);
                }
            }
        }
        println!("{}", line);
    }
}

fn rename_user<R>(new_name: String, _: &mut R, user_map: &UIDMap<R>, username: &mut String) where R: Read+Write{
    let mut map = user_map.lock().unwrap();
    let bor = map.remove(username).expect("User not Found!");
    map.insert(new_name.clone(),bor);

    println!("User {} has been renamed to {}!", username, new_name);
    username.clone_from(&new_name);
}
