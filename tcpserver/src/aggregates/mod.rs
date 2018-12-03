pub mod counter;
pub mod leftrightcounter;

pub trait StateLoader {
    type TState;
    type TEvents;

    fn apply(&self, state: &mut Self::TState, event: Self::TEvents);
}

pub trait CommandHandler {
    type TState;
    type TCommands;
    type TEvents;
    type TErrors;

    fn eval(&self, state: Self::TState, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors>;
}