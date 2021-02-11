use std::prelude::v1::*;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;

pub use Pass::*;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

///
/// The type being passed between the Server Threads for reading incoming messages and the Threads for sending messages
///
pub type ServerReceiver<R> = Receiver<Pass<R>>;

pub enum Pass<R> {
    Function(Box<dyn FnOnce(&mut R,&UIDMap<R>, &mut String) -> Pass<R> + Send>),
    NOOP,
    CloseConnection
}

pub type Commands<AT> = Vec<Rc<dyn super::Command<AT>>>;

pub type UIDMap<A> = Arc<Mutex<HashMap<String, Vec<Sender<Pass<A>>>>>>;
