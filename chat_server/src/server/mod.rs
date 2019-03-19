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
    if let Ok((tx, rx, username, stream)) = connection_init(stream, &user_map) {
        let stream_copy = stream.try_clone()?;

        thread::spawn(move || connection_send_loop(stream_copy, rx, &user_map, username));
        connection_receive_loop(stream, tx);
    }
    Ok(())
}

type InitReturn = Result<(Sender<Pass<TcpStream>>, ServerReceiver<TcpStream>, String, TcpStream)>;

fn connection_init(stream: TcpStream, user_map: &UIDMap<TcpStream>) -> InitReturn {
    use crate::network::connection;
    use connection::init;

    if let Ok(ServerConnection { stream, username }) = init::server_connection(connection::ServerInitInput {}, stream) {
        let mut uid = UserIdentifier::new(&username);

        let (tx, rx): (_, ServerReceiver<TcpStream>) = channel();

        uid.connections.push(tx.clone());
        user_map.lock()
            .map(|mut map| map.insert(username.clone(), Vec::new()))
            .map(|_ok| (tx, rx, username, stream))
            .map_err(|_err| Error::new(ErrorKind::Other, "Error while locking mutex for UserMap!"))
    } else {
        Err(Error::new(ErrorKind::InvalidData, "Expected Server got something else"))
    }
}

fn connection_send_loop<R>(mut stream: R, rx: ServerReceiver<R>, user_map: &UIDMap<R>, mut username: String) where R: Read + Write {
    use  crate::lib::api::alias::Pass::*;
    loop {
        match rx.recv() {
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
            Ok(NOOP) => {}
            Ok(CloseConnection) => {
                let mut map = user_map.lock().unwrap();
                map.remove(&username);
                break;
            }
            Ok(Function(tmp)) => {
                tmp(&mut stream, user_map, &mut username);
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
                let new_name: String = line.trim_start_matches("!rename ").to_string();

                let tmp_rename = |a: &mut R, b: &UIDMap<R>, c: &mut String| { rename_user(new_name, a, b, c) };
                ;

                if let Err(e) = tx.send(Pass::Function(Box::new(tmp_rename))) {
                    eprintln!("{}", e);
                }
            }
        }
        println!("{}", line);
    }
}

fn rename_user<R>(new_name: String, _: &mut R, user_map: &UIDMap<R>, username: &mut String) -> Pass<R> where R: Read + Write {
    let old_name = username.clone();
    match user_map.lock()
        .map(|mut map| {
            map.remove(username)
                .map(|user_object| {
                    map.insert(new_name.clone(), user_object);
                    username.clone_from(&new_name);
                });
        }) {
        Ok(_ok) => {println!("User successfully renamed from {} to {}!",old_name,username)}
        Err(err) => {eprintln!("Failed to rename User from {} to {} because of {}",old_name,new_name,err)}

    }
    Pass::NOOP
}
