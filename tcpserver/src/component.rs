use std::cell::RefCell;
use std::fmt;

use logging::*;
use sink::*;

/// An aggregate is a container which owns a source of truth or data set
pub trait Actor {
    type TCommands;
    type TEvents;
    type TErrors;

    // fn update(&mut self, event: Self::TEvents);
    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors>;
}

pub struct System<TContext, TSystem>(TContext, TSystem);

impl<TContext, TSystem> System<TContext, TSystem> {
    pub fn ctx(&self) -> &TContext {
        &self.0
    }

    pub fn system(&self) -> &TSystem {
        &self.1
    }
}

pub trait SystemDef<TContext, TSystem> {
    fn bind(ctx: TContext) -> System<TContext, TSystem>;
}

impl<TSystem, TContext, TCommands, TEvents, TErrors> SystemDef<TContext, RefCell<TSystem>>
    for RefCell<TSystem>
where
    TContext: Dispatcher<LoggingEvents, ()> + Dispatcher<TEvents, ()> + Dispatcher<TErrors, ()>,
    // T: Actor<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors> + Initializable,
    TSystem: Sink<TInput=TCommands, TResult=Result<TEvents, TErrors>> + Initializable,
{
    fn bind(ctx: TContext) -> System<TContext, RefCell<TSystem>> {
        System(ctx, Self::default())
    }
}

pub enum DispatchErrors<TEvent, TError> {
    Event (TEvent),
    Error (TError),
}

impl<TContext, TSystem, TCommands, TEvents, TErrors> Sink
    for System<TContext, RefCell<TSystem>>
where
    TCommands: fmt::Debug,
    TEvents: fmt::Debug,
    TErrors: fmt::Debug,
    TContext: Dispatcher<LoggingEvents, ()> + Dispatcher<TEvents, ()> + Dispatcher<TErrors, ()>,
    // TSystem: Actor<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>,
    TSystem: Sink<TInput=TCommands, TResult=Result<TEvents, TErrors>>,
{
    type TInput = TCommands;
    type TResult = ();

    fn send(&self, input: Self::TInput) -> Self::TResult {
        // let mut system = self.system().borrow_mut();
        let system = self.system().borrow();
        self.ctx().dispatch(trace!("Command: {:?}", input));
        match system.send(input) {
            Ok(event) => {
                self.ctx().dispatch(warn!("Event: {:?}", event));
                self.ctx().dispatch(event);
                // system.update(event);
            }
            Err(err) => {
                self.ctx().dispatch(error!("Error: {:?}", err));
                self.ctx().dispatch(err);
            }
        }
        ()
    }
}


// pub type Context<TEvents> = (Sink<TInput=LoggingEvents, TResult=()>, Sink<TInput=TEvents, TResult=()>);

// pub type AggregateSystem<TContext, TSystem> = System<TContext, TSystem>;

// impl<T, TContext, TCommands, TEvents, TErrors> SystemDef<TContext, RefCell<T>>
//     for RefCell<T>
// where
//     TContext: Context<TEvents>,
//     T: Actor<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors> + Initializable,
// {
//     fn bind(ctx: TContext) -> AggregateSystem<TContext, RefCell<T>> {
//         System(ctx, Self::default())
//     }
// }

// impl<TContext, TSystem, TCommands, TEvents, TErrors> Sink
//     for System<TContext, RefCell<TSystem>>
// where
//     TCommands: fmt::Debug,
//     TEvents: fmt::Debug,
//     TErrors: fmt::Debug,
//     TContext: Context<TEvents>,// Dispatcher<LoggingEvents, ()> + Dispatcher<TEvents, ()>,
//     TSystem: Actor<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>,
// {
//     type TInput = TCommands;
//     type TResult = ();

//     fn send(&self, input: Self::TInput) -> Self::TResult {
//         let mut system = self.system().borrow_mut();
//         self.ctx().dispatch(trace!("Command: {:?}", input));
//         match system.handle(input) {
//             Ok(event) => {
//                 self.ctx().dispatch(warn!("Event: {:?}", event));
//                 system.update(event);
//             }
//             Err(err) => {
//                 self.ctx().dispatch(error!("Error: {:?}", err));
//             }
//         }
//         ()
//     }
// }

// pub type AggregateContext<TEvents>: Dispatcher<LoggingEvents, ()> + Dispatcher<TEvents, ()>;

// pub enum AggregateSinkContext<TEvents> {
//     Event (TEvents),
//     Logging (LoggingEvents),
// }

// where
//     TLoggingSink: Sink<TInput=LoggingEvents, TResult=()>,
//     TEventSink: Sink<TInput=TEvents, TResult=()>
// pub struct Context<
//     TEvents,
//     TLoggingSink: Sink<TInput=LoggingEvents, TResult=()>,
//     TEventSink: Sink<TInput=TEvents, TResult=()>
// >(TLoggingSink, TEventSink);

// impl<TEvents, TLoggingSink, TEventSink> AggregateContext<TEvents, TLoggingSink, TEventSink>
// where
//     TLoggingSink: Sink<TInput=LoggingEvents, TResult=()>,
//     TEventSink: Sink<TInput=TEvents, TResult=()>,
// {
//     pub fn event(&self, input: TEvents) {
//         self.1.send(input)
//     }

//     pub fn logging(&self, input: LoggingEvents) {
//         self.0.send(input)
//     }
// }
// (Sink<TInput=LoggingEvents, TResult=()>, Sink<TInput=TEvents, TResult=()>),//Context<TEvents, _, _>,// Dispatcher<LoggingEvents, ()> + Dispatcher<TEvents, ()>,
    // TContext: Dispatcher<LoggingEvents, ()> + Dispatcher<TEvents, ()>,

// pub type Context<TEvents, TLoggingSink, TEventSink> = (Sink<TInput=LoggingEvents, TResult=()>, Sink<TInput=TEvents, TResult=()>);
// pub type Context<TEvents, TLoggingSink: Sink<TInput=LoggingEvents, TResult=()>, TEventSink: Sink<TInput=TEvents, TResult=()>> = (TLoggingSink, TEventSink);

// pub struct Context<TEvents, TLoggingSink: Sink<TInput=LoggingEvents, TResult=()>, TEventSink: Sink<TInput=TEvents, TResult=()>> {
//     logging: TLoggingSink,
//     events: TEventSink,
// }
// Sink<TInput=LoggingEvents, TResult=()>, Sink<TInput=TEvents, TResult=()>);

// pub type AggregateSystem<TContext, TSystem> = System<( Sink<TInput=LoggingEvents, TResult=()>, Sink<TInput=TEvents, TResult=()> ), TSystem>;
