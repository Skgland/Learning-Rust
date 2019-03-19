use std::string::String;
use std::vec::Vec;
use std::option::Option;

pub use self::serialization::*;

///
/// A Trait for defining Chat Server/Client Commands
///

pub trait Command<AT> {


    ///
    /// the primary name for this command
    ///
    fn name(&self) -> &'static str;

    ///
    /// a list of alternative names for this command
    ///
    fn alias(&self) -> Vec<&'static str>;

    ///
    /// the action performed when the command gets called
    ///
    fn run(&self,chat_line: String) -> Option<String>;


    ///
    /// the help message to be displayed when the help command is used to list all commands
    ///
    fn help_all(&self, chat_line: Option<String>) -> Vec<String>;

    ///
    /// the help message to be displayed when help is requested for explicitly this command
    ///
    fn help_explicit(&self, chat_line: Option<String>) -> Vec<String>;

    ///
    /// the syntax for this command to present auto-completion
    ///
    fn syntax(&self);

}

pub trait SidePair {

    type OTHER: SidePair;
}

pub trait PacketFactory<AT> where AT: SidePair {
    fn summon(&self) -> &dyn Packet<AT>;
}

pub trait Packet<AT> where Self: Serializable, AT: SidePair {
    fn handle(&self, side: AT);
}
