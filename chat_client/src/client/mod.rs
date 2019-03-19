use std::prelude::v1::*;
use std::net::TcpStream;
use std::io::Result;
use std::io::ErrorKind;

use crate::lib::prelude::v1::*;
use std::thread::JoinHandle;
use std::thread;
use std::io::Error;

const SERVER_QUESTION: &str = "Please enter <Host>:<Port> to connect or type Quit to quit!";

#[allow(clippy::option_map_unit_fn)]
#[allow(clippy::result_map_unit_fn)]
pub fn client_entry_point() {
    crate::lib::common_entry();

    let commands: Commands<Client> = Vec::new();

    if let Err(e) = client_pre_init().and_then(|a| client_init(a, commands)).map(client_start){
        eprintln!("{}",e);
    }
}

fn client_start(client: Client) ->! {
    let handle = thread::spawn(move || receive_loop());
    input_loop(client, handle);
}

fn client_pre_init() -> Result<TcpStream> {
    loop {
        let response = askln(SERVER_QUESTION);
        match response {
            Ok(mut server) => {
                match server.as_str() {
                    "Quit" => return Err(Error::new(ErrorKind::ConnectionAborted, "User Quit")),
                    "" => server = String::from("localhost:8888"),
                    _ => {}
                }
                match TcpStream::connect(server.clone()) {
                    Ok(stream) => break Ok(stream),
                    Err(e) => {
                        eprintln!("Couldn't connect! \"{}\"", e)
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed Reading Server Address \"{}\"", e);
            }
        }
    }
}

pub fn client_init(stream: TcpStream, commands: Commands<Client>) -> Result<Client> {
    use crate::network::connection;
    use connection::init;

    if let Ok(c) = init::client_connection(connection::ClientInitInput{commands}, stream){
        Ok(c)
    }else{
        Err(Error::new(ErrorKind::InvalidData,"Expected Client got something else"))
    }
}


pub fn input_loop<A>(Client { mut stream, .. }: Client, _handle: JoinHandle<A>) -> ! {
    println!("Entering Client Input Loop!");
    loop {
        //import part macro
        use crate::lib::part;

        let mut message = part!(message in (Ok(message) = ask("")) else panic!("Error while asking for input!")) ;

        message.serialize(&mut ReadWrite::WRITE { writer: &mut stream }).expect("Fuck!");
    }
}

pub fn receive_loop() -> Result<()> {
    println!("Starting Client Receive Loop!");
    //FIXME
    println!("Receive Loop unimplemented!");
    Ok(())
}
