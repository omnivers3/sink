use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;

use logging::*;
use sink::*;

pub trait Provider<TOutput> {
    fn next(&self) -> TOutput;
}

/// An aggregate is a container which owns a source of truth or data set
pub trait AggregateRoot {
    type TCommands;
    type TEvents;
    type TErrors;

    fn update(&mut self, event: Self::TEvents);
    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors>;
}

pub struct SystemContext {}

pub struct AggregateSystem<T, TContext, TCommands, TEvents, TErrors>
where
    T: AggregateRoot,
{
    _commands: PhantomData<TCommands>,
    _events: PhantomData<TEvents>,
    _errors: PhantomData<TErrors>,
    context: TContext,
    aggregate: RefCell<T>,
}

impl<T, TContext, TCommands, TEvents, TErrors>
    AggregateSystem<T, TContext, TCommands, TEvents, TErrors>
where
    TContext: Sink<TInput = LoggingEvents, TResult = ()>,
    T: AggregateRoot + Default,
{
    pub fn new(context: TContext) -> Self {
        AggregateSystem {
            _commands: PhantomData,
            _events: PhantomData,
            _errors: PhantomData,
            context,
            aggregate: RefCell::<T>::default(),
        }
    }
}

impl<T, TContext, TCommands, TEvents, TErrors> Sink
    for AggregateSystem<T, TContext, TCommands, TEvents, TErrors>
where
    TCommands: fmt::Debug,
    TEvents: fmt::Debug,
    TErrors: fmt::Debug,
    TContext: Dispatcher<LoggingEvents, ()>,
    T: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>,
{
    type TInput = TCommands;
    type TResult = ();

    fn send(&self, input: Self::TInput) -> Self::TResult {
        let mut aggregate = self.aggregate.borrow_mut();
        info!("Command: {:?}", input);
        // self.context.dispatch(LoggingEvents::Trace(format!("Command: {:?}", input)));
        let result = aggregate.handle(input);
        match result {
            Ok(event) => {
                info!("Event: {:?}", event);
                // self.context.dispatch(LoggingEvents::Trace(format!("Event: {:?}", event)));
                aggregate.update(event);
            }
            Err(err) => {
                error!("Error: {:?}", err);
                // self.context.dispatch(LoggingEvents::Trace(format!("Error: {:?}", err)));
            }
        }
        ()
    }
}

pub trait IntoSystem<T, TCommands, TEvents, TErrors>
where
    T: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors> + Default,
{
    fn to_system<TContext>(
        context: TContext,
    ) -> AggregateSystem<T, TContext, TCommands, TEvents, TErrors>
    where
        TContext: Sink<TInput = LoggingEvents, TResult = ()>,
    {
        AggregateSystem::new(context)
    }
}

impl<T, TCommands, TEvents, TErrors> IntoSystem<T, TCommands, TEvents, TErrors> for T where
    T: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors> + Default
{}
