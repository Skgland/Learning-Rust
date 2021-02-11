use std::prelude::v1::*;
use std::net::TcpStream;
use std::io::Result;
use std::io::ErrorKind;

use std::thread::JoinHandle;
use std::thread;
use std::io::Error;

use crate::lib::addons::alias::*;
use crate::network::serialization::*;
use crate::lib::InitState;
use crate::lib;
use chat_base::structs::Client;
use std::result::Result::Ok;
use crate::lib::Side;


const SERVER_QUESTION: &str = "Please enter <Host>:<Port> to connect or type Quit to quit!";

pub fn client_entry_point() {
    base_lib::common_entry();

    let commands: Commands<Client> = Vec::new();

    if let Ok(stream) = client_pre_init() {
        if let Err(e) = client_init(&stream){
            eprintln!("{}",e);
            return;
        }
        println!("Starting Client");
        client_start(Client{stream,commands})
    }
}

fn client_start(client: Client) -> ! {
    let handle = thread::spawn(move || receive_loop());
    input_loop(client, handle);
}

fn client_pre_init() -> Result<TcpStream> {
    loop {
        let response = base_lib::commandline::askln(SERVER_QUESTION);
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

pub fn client_init(stream: &TcpStream) -> Result<()> {
    use crate::network::connection::*;

    let copy1 = &mut stream.try_clone()?;
    let copy2 = &mut stream.try_clone()?;

    let to_server = &mut ReadWrite::WRITE { writer: copy1 };
    let to_client = &mut ReadWrite::READ { reader: copy2 };

    let mut state = InitState::new(Side::Client);
    let mut channel = ChannelPair::ClientPair {impl_to_server:to_server,impl_to_client:to_client};

    println!("Init Setup complete!");

    let init_protocol = lib::init_protocol();
    println!("Initialized Protocol");
    init_protocol.run_protocol(&mut state, &mut channel)?;

    Ok(())
}


pub fn input_loop<A>(Client { mut stream, .. }: Client, _handle: JoinHandle<A>) -> ! {
    println!("Entering Client Input Loop!");
    loop {
        //import part macro
        use base_lib::part;

        let mut message = part!(message in (Ok(message) = base_lib::commandline::ask("")) else panic!("Error while asking for input!"));

        message.serialize(&mut ReadWrite::WRITE { writer: &mut stream }).expect("Fuck!");
    }
}

pub fn receive_loop() -> Result<()> {
    println!("Starting Client Receive Loop!");
    //FIXME
    println!("Receive Loop unimplemented!");
    Ok(())
}
