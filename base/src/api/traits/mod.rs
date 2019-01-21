use std::string::String;
use std::vec::Vec;
use std::option::Option;

pub mod serialization;

pub use self::serialization::*;

pub trait Command<AT> {

    fn packets(&self) -> Vec<&dyn PacketFactory<AT>>;

    fn alias(&self) -> Vec<&'static str>;

    fn name(&self) -> &'static str;

    fn run(&self,chat_line: String) -> Option<String>;

    fn help(&self,chat_line: Option<String>) -> Vec<String>;

    fn syntax_tree(&self);

    fn only_in_debug_mode(&self) -> bool;

}

pub trait SidePair {

    type OTHER: SidePair;
}

pub trait PacketFactory<AT> where AT: SidePair {
    fn summon(&self) -> &dyn Packet<AT>;
}

pub trait Packet<AT> where Self: Serializeable, AT: SidePair {
    fn handle(&self, side: AT);
}
