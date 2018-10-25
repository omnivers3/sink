use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;

use sink::*;
use log::*;

/// An aggregate is a container which owns a source of truth or data set
pub trait IAggregate {
    type TCommands;
    type TEvents;
    type TErrors;

    fn update(&mut self, event: Self::TEvents);
    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors>;
}

pub struct AggregateHarness<T, TCommands, TEvents, TErrors>
where
    T: IAggregate,
{
    _commands: PhantomData<TCommands>,
    _events: PhantomData<TEvents>,
    _errors: PhantomData<TErrors>,
    aggregate: RefCell<T>,
}

impl<T, TCommands, TEvents, TErrors> AggregateHarness<T, TCommands, TEvents, TErrors>
where
    T: IAggregate + Default,
{
    pub fn new() -> Self {
        AggregateHarness {
            _commands: PhantomData,
            _events: PhantomData,
            _errors: PhantomData,
            aggregate: RefCell::<T>::default(),
        }
    }
}

impl<T, TCommands, TEvents, TErrors> ISink for AggregateHarness<T, TCommands, TEvents, TErrors>
where
    TCommands: fmt::Debug,
    TEvents: fmt::Debug,
    TErrors: fmt::Debug,
    T: IAggregate<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors>
{
    type TInput = TCommands;
    type TResult = ();

    fn send(&self, input: Self::TInput) -> Self::TResult {
        let mut aggregate = self.aggregate.borrow_mut();
        debug!("Command: {:?}", input);
        let result = aggregate.handle(input);
        match result {
            Ok (event) => {
                debug!("Event: {:?}", event);
                aggregate.update(event);
            }
            Err (err) => {
                debug!("Error: {:?}", err);
            }
        }
        ()
    }
}

pub trait IIntoHarness<T, TCommands, TEvents, TErrors>
where
    T: IAggregate<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors> + Default
{
    fn to_harness() -> AggregateHarness<T, TCommands, TEvents, TErrors> {
        AggregateHarness::new()
    }
}

impl<T, TCommands, TEvents, TErrors> IIntoHarness<T, TCommands, TEvents, TErrors> for T
where
    T: IAggregate<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors> + Default
{}

pub trait IInitialized: Default {
    type TState;

    fn init(state: Self::TState) -> Self {
        let mut default = Self::default();
        default.apply(state);
        default
    }

    fn apply(&mut self, state: Self::TState);
}
