use std::cell::RefCell;
use std::fmt;

use logging::*;
use sink::*;

pub trait ActorState<TConfig> {
    fn from(config: &TConfig) -> Self;
}

pub trait Actor {
    type TState;
    type TCommands;
    type TEvents;
    type TErrors;
    type TResult;

    fn handle(&self,
        state: &mut Self::TState,
        command: Self::TCommands,
        events: impl Sink<TInput=Self::TEvents, TResult=()>,
        errors: impl Sink<TInput=Self::TErrors, TResult=()>
    ) -> Self::TResult;
}

// pub trait Runtime<TContext> {
//     fn run(self, ctx: TContext);
// }

// pub struct RuntimeWrapper<TContext, TRuntime>
// where
//     TRuntime: Runtime<TContext>,
// {
//     ctx: TContext,
//     runtime: TRuntime,
// }

// pub trait RuntimeExec {
//     fn run(self);
// }

// impl<TContext, TRuntime> RuntimeExec for RuntimeWrapper<TContext, TRuntime>
// where
//     TRuntime: Runtime<TContext>,
// {
//     fn run(self) {
//         self.runtime.run(self.ctx)
//     }
// }

// pub trait RuntimeDef<TContext, TRuntime>
// where
//     TRuntime: Runtime<TContext>,
// {
//     fn bind(self, ctx: TContext) -> RuntimeWrapper<TContext, TRuntime>;
// }

// impl<TContext, TRuntime> RuntimeDef<TContext, TRuntime> for TRuntime
// where
//     TRuntime: Runtime<TContext>,
// {
//     fn bind(self, ctx: TContext) -> RuntimeWrapper<TContext, TRuntime> {
//         RuntimeWrapper {
//             ctx,
//             runtime: self,
//         }
//     }
// }

pub struct System<TContext, TSystem> {
    context: TContext,
    system: TSystem,
}

impl<TContext, TSystem> System<TContext, TSystem> {
    pub fn new(context: TContext, system: TSystem) -> Self {
        System {
            context,
            system,
        }
    }
    
    pub fn context(&self) -> &TContext {
        &self.context
    }

    pub fn system(&self) -> &TSystem {
        &self.system
    }
}

pub trait SystemDef<TContext, TSystem> {
    fn bind(ctx: TContext) -> System<TContext, TSystem>;
}

impl<TSystem, TContext, TCommands, TEvents, TErrors> SystemDef<TContext, RefCell<TSystem>>
    for RefCell<TSystem>
where
    TContext: Dispatcher<LoggingEvents> + Dispatcher<TEvents> + Dispatcher<TErrors>,
    // T: Actor<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors> + Initializable,
    TSystem: Sink<TInput=TCommands, TResult=Result<TEvents, TErrors>> + Default,
{
    fn bind(ctx: TContext) -> System<TContext, RefCell<TSystem>> {
        System::new(ctx, Self::default())
    }
}

impl<TContext, TSystem, TCommands, TEvents, TErrors> Sink
    for System<TContext, RefCell<TSystem>>
where
    TCommands: fmt::Debug,
    TEvents: fmt::Debug,
    TErrors: fmt::Debug,
    TContext: Dispatcher<LoggingEvents> + Dispatcher<TEvents> + Dispatcher<TErrors>,
    // TSystem: Actor<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>,
    TSystem: Sink<TInput=TCommands, TResult=Result<TEvents, TErrors>>,
{
    type TInput = TCommands;
    type TResult = ();

    fn send(&self, input: Self::TInput) -> Self::TResult {
        let ctx = self.context();
        // let mut system = self.system().borrow_mut();
        let system = self.system().borrow();
        ctx.dispatch(trace!("Command: {:?}", input));
        match system.send(input) {
            Ok(event) => {
                ctx.dispatch(warn!("Event: {:?}", event));
                ctx.dispatch(event);
                // system.update(event);
            }
            Err(err) => {
                ctx.dispatch(error!("Error: {:?}", err));
                ctx.dispatch(err);
            }
        }
        ()
    }
}

// pub enum DispatchErrors<TEvent, TError> {
//     Event (TEvent),
//     Error (TError),
// }

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
