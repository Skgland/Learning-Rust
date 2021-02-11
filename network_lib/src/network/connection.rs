use crate::serialization::*;

pub enum ChannelPair<'a> {
    ServerPair { impl_to_server: &'a mut ReadWrite<'a>, impl_to_client: &'a mut ReadWrite<'a> },
    ClientPair { impl_to_server: &'a mut ReadWrite<'a>, impl_to_client: &'a mut ReadWrite<'a> },
}

use self::ChannelPair::*;

impl<'a> ChannelPair<'a> {

    pub fn towards_server(&mut self) -> &mut ReadWrite<'a> {
        match self {
            ServerPair { impl_to_server, .. } => {
                impl_to_server
            }
            ClientPair { impl_to_server, .. } => {
                impl_to_server
            }
        }
    }
    pub fn towards_client(&mut self) -> &mut ReadWrite<'a> {
        match self {
            ServerPair { impl_to_client, .. } => {
                impl_to_client
            }
            ClientPair { impl_to_client, .. } => {
                impl_to_client
            }
        }
    }

    pub fn towards_other_side(&mut self) -> &mut ReadWrite<'a> {
        match self {
            ServerPair { impl_to_client, .. } => {
                impl_to_client
            }
            ClientPair { impl_to_server, .. } => {
                impl_to_server
            }
        }
    }

    pub fn towards_this_side(&mut self) -> &mut ReadWrite<'a> {
        match self {
            ServerPair { impl_to_server, .. } => {
                impl_to_server
            }
            ClientPair { impl_to_client, .. } => {
                impl_to_client
            }
        }
    }
}


