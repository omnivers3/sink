use component::AggregateRoot;
use sink::Initializable;

use net;

#[derive(Debug)]
pub enum Commands {
    Socket(net::Commands),
}

#[derive(Debug)]
pub enum Events {
    Socket(net::Events),
}

#[derive(Debug)]
pub enum Errors {
    Socket(net::Errors),
}

pub struct State {
    socket: net::State,
}

pub struct Component {
    socket: net::Component,
}

impl Default for Component {
    fn default() -> Self {
        Component {
            socket: net::Component::default(),
        }
    }
}

impl Initializable for Component {
    type TState = State;

    fn apply(&mut self, state: State) {
        self.socket.apply(state.socket);
    }
}

impl AggregateRoot for Component {
    type TCommands = Commands;
    type TEvents = Events;
    type TErrors = Errors;

    fn update(&mut self, event: Self::TEvents) {
        match event {
            Events::Socket(event) => self.socket.update(event),
        }
    }

    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors> {
        match command {
            Commands::Socket(command) => self
                .socket
                .handle(command)
                .map(Events::Socket)
                .map_err(Errors::Socket),
        }
    }
}
