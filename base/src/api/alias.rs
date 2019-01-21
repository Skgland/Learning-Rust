use super::traits::Command;
use std::prelude::v1::*;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;

pub use Pass::*;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub type ServerReceiver<R> = Receiver<Pass<R>>;

pub enum Pass<R> {
    Function(Box<dyn FnOnce(&mut R,&UIDMap<R>, &mut String) -> () + Send>),
    CloseConnection
}

pub type Commands<AT> = Vec<Rc<Command<AT>>>;

pub type UIDMap<A> = Arc<Mutex<HashMap<String, Vec<Sender<Pass<A>>>>>>;
