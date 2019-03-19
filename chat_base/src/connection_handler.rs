use std::io::Read;
use std::io::Write;
use std::io::Result;
use std::net::TcpStream;
use std::marker::PhantomData;
use std::thread;
use std::thread::JoinHandle;

struct TcpConnectionHandler<'a,SIDE> {
    read_thread: Option<JoinHandle<()>>,
    write: &'a mut TcpStream,
    _side: PhantomData<SIDE>
}

impl<Side> Drop for TcpConnectionHandler<'_, Side> {
    fn drop(&mut self) {
        if let Some(handle) = self.read_thread.take() {
            if let Err(e) = handle.join() {
                eprintln!("{:?}", e)
            }
        }
    }
}

impl<'a>  TcpConnectionHandler<'a,Client>{
    fn create_handler(read: &'a mut TcpStream, write: &'a mut TcpStream) -> Self {
        let mut r_copy = read.try_clone().expect("Couldn't Clone TcpStream");
        let read_thread: JoinHandle<()> = thread::spawn(move || {
            read_loop(&mut r_copy);
        });

        TcpConnectionHandler { read_thread: Some(read_thread), write, _side: PhantomData }
    }
}

trait ConnectionHandler<'a,AT> {
    type READ: Read;
    type WRITE: Write;
    type INIT_PARAMETER;

    fn create_handler(parmas:Self::INIT_PARAMETER);
    fn send(&mut self, packet: &mut dyn Packet<AT>) -> Result<()>;
}

impl<'a, AT> ConnectionHandler<'a, AT> for TcpConnectionHandler<'a, AT>{
    type READ = TcpStream;
    type WRITE = TcpStream;
    type INIT_PARAMETER = (TcpStream,);

    fn create_handler(parmas: <Self as ConnectionHandler<'a, AT>>::INIT_PARAMETER) {
        unimplemented!()
    }

    default fn send(&mut self, packet: &mut dyn Packet<AT>) -> Result<()> {
        packet.serialize(&mut ReadWrite::WRITE { writer: &mut self.write })?;
        Ok(())
    }
}

fn read_loop(read:&mut Read){
    unimplemented!()
}

