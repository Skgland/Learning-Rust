use crate::lib::api::traits::serialization::Serializable;

pub enum Protocol<State, A: StateAccess<State>> {
    END,
    Chain(Box<Protocol<State, A>>, Box<Protocol<State, A>>),
    Conditional(Box<Fn(&State) -> bool>, Box<Protocol<State, A>>, Box<Protocol<State, A>>),
    Update(Box<Fn(&mut State) -> ()>),
    Transfer(Destination, A),
}


pub trait StateAccess<State> {
    fn get_access(&self, state: &State) -> &mut dyn Serializable;
}

pub enum Destination {
    ToServer,
    ToClient,
}

impl<State, A: StateAccess<State>> Protocol<State, A> {
    pub fn run_protocol(&self, state: &mut State, connection: &mut super::connection::ChannelPair) -> std::io::Result<()>{
        let mut order = Vec::new();
        order.push(self);

        while let Some(protocol) = order.pop() {
            match protocol {
                Protocol::END => {}
                Protocol::Transfer(Destination::ToServer, accessor) => {
                    accessor.get_access(state).serialize(connection.to_server())?;
                }
                Protocol::Transfer(Destination::ToClient, accessor) => {
                    accessor.get_access(state).serialize(connection.to_client())?;
                }
                Protocol::Update(update) => {
                    update(state);
                }
                Protocol::Chain(first, second) => {
                    order.push(second);
                    order.push(first);
                }
                Protocol::Conditional(condition,when,otherwise) => {
                    if condition(state) {
                        order.push(when);
                    }else {
                        order.push(otherwise);
                    }
                }
            }
        }
        Ok(())
    }
}



