use crate::network::serialization::Serializable;
use crate::network::protocol::Protocol::Update;

pub enum Protocol<State, A: StateAccess<State>> {
    Chain(Vec<Protocol<State, A>>),
    Conditional(Box<dyn Fn(&State) -> bool>, Box<Protocol<State, A>>, Box<Protocol<State, A>>),
    Update(Box<dyn Fn(&mut State) -> std::io::Result<()>>),
    Exchange(A, A),
    Transfer(Destination, A),
}


pub trait StateAccess<State> {
    fn get_access<'b>(&self, state: &'b mut State) -> std::io::Result<&'b mut dyn Serializable>;
}

pub enum Destination {
    ToServer,
    ToClient,
}

impl<State, A: StateAccess<State>> Protocol<State, A> {
    #[allow(non_snake_case)]
    pub fn End() -> Self {
        Update(Box::new(|_| Ok(())))
    }

    pub fn run_protocol(&self, state: &mut State, connection: &mut super::connection::ChannelPair) -> std::io::Result<()> {
        let mut order = Vec::new();
        order.push(self);

        while let Some(protocol) = order.pop() {
            match protocol {
                Protocol::Transfer(Destination::ToServer, accessor) => {
                    println!("Transferring to Server");
                    match accessor.get_access(state) {
                        Err(e) => Err(e),
                        Ok(access) => access.serialize(connection.towards_server()),
                    }?
                }
                Protocol::Transfer(Destination::ToClient, accessor) => {
                    println!("Transferring to Client");
                    match accessor.get_access(state) {
                        Ok(access) => access.serialize(connection.towards_client()),
                        Err(e) => Err(e)
                    }?
                }
                Protocol::Exchange(sending, receiving) => {
                    println!("Sending for exchange");
                    match sending.get_access(state) {
                        Ok(sending) => {
                            println!("Serializing");
                            sending.serialize(connection.towards_other_side())?
                        },
                        Err(e) => return Err(e)
                    }
                    println!("Receiving for exchange");
                    match receiving.get_access(state) {
                        Ok(receiving) => receiving.serialize(connection.towards_this_side())?,
                        Err(e) => return Err(e)
                    }

                }
                Protocol::Update(update) => {
                    println!("Updating");
                    update(state)?;
                }
                Protocol::Chain(content) => {
                    println!("Chaining");
                    order.extend(content.iter().rev());
                }
                Protocol::Conditional(condition, when, otherwise) => {
                    println!("Deciding");
                    if condition(state) {
                        order.push(when);
                    } else {
                        order.push(otherwise);
                    }
                }
            }
        }
        Ok(())
    }
}



