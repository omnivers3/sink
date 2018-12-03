// Example composite aggregate which borrows functionality from other primitives

use super::{ StateLoader, CommandHandler };
use super::counter;

#[derive(Debug, Eq, PartialEq)]
pub struct Config {
    left: counter::Config,
    right: counter::Config,
}

#[derive(Debug, Eq, PartialEq)]
pub struct State {
    left: counter::State,
    right: counter::State,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Commands {
    Left (counter::Commands),
    Right (counter::Commands),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Events {
    Left (counter::Events),
    Right (counter::Events),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Errors {
    Left (counter::Errors),
    Right (counter::Errors),
}

impl Config {
    pub fn new(left: counter::Config, right: counter::Config) -> Self {
        Config { left, right }
    }
}

impl StateLoader for Config {
    type TState = State;
    type TEvents = Events;

    fn apply(&self, state: &mut Self::TState, event: Self::TEvents) {
        match event {
            Events::Left (event) => self.left.apply(&mut state.left, event),
            Events::Right (event) => self.right.apply(&mut state.right, event),
        }
    }
}

impl CommandHandler for Config {
    type TState = State;
    type TCommands = Commands;
    type TEvents = Events;
    type TErrors = Errors;

    fn eval(&self, state: State, command: Commands) -> Result<Events, Errors> {
        match command {
            Commands::Left (command) => self.left.eval(state.left, command).map(Events::Left).map_err(Errors::Left),
            Commands::Right (command) => self.right.eval(state.right, command).map(Events::Right).map_err(Errors::Right),
        }
    }
}
