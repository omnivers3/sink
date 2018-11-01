use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;

use logging::*;
use sink::*;

pub trait Provider<TOutput> {
    fn next(&self) -> TOutput;
}

// pub type System<TContext: Sized, TSystem> = (TContext, TSystem);
pub struct Actor<TContext, TActor>(TContext, TActor);

pub trait ActorDef<TContext, TActor> {
    fn bind(ctx: TContext) -> Actor<TContext, TActor>;
}

/// An aggregate is a container which owns a source of truth or data set
pub trait AggregateRoot {
    type TCommands;
    type TEvents;
    type TErrors;

    fn update(&mut self, event: Self::TEvents);
    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors>;
}

// pub type AggregateSystem<TContext, TSystem>
// where
//     for<'a> TContext: Dispatcher<LoggingEvents<'a>, ()>,
//     TSystem: AggregateRoot + Initializable,
// = System<TContext, TSystem>;

pub type AggregateActor<'a, TContext, TActor>
where
    TContext: Dispatcher<LoggingEvents<'a>, ()>,
    TActor: AggregateRoot + Initializable,
= Actor<TContext, TActor>;

// struct LifetimeWrapper<'a, TInner> {
//     _lifetime: PhantomData<&'a ()>,
//     inner: TInner,
// }

impl<'a, T, TContext, TCommands, TEvents, TErrors> ActorDef<TContext, RefCell<T>>
    for RefCell<T>
where
    // for<'a> TContext: Dispatcher<LoggingEvents<'a>, ()>,
    TContext: Dispatcher<LoggingEvents<'a>, ()>,
    // Self: Initializable,
    T: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors> + Initializable,
{
    fn bind(ctx: TContext) -> AggregateActor<'a, TContext, RefCell<T>> {
    // fn bind(ctx: TContext) -> System<TContext, Self> {
        Actor(ctx, Self::default())//Self::default())
    }
}

impl<'a, TContext, TActor, TCommands, TEvents, TErrors> Sink
//     for T
// where
//     T: System<TContext, AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>>,
    for Actor<TContext, RefCell<TActor>>//SystemDef<TContext>
where
    TCommands: 'a + fmt::Debug,
    TEvents: fmt::Debug,
    TErrors: fmt::Debug,
    TContext: Dispatcher<LoggingEvents<'a>, ()>,
    TActor: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>,
    // TActor: AggregateActor<'a, TContext, T>,
// where
//     T: SystemDef<TContext> + AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>,
{
    type TInput = TCommands;
    type TResult = ();

    fn send(&self, input: Self::TInput) -> Self::TResult {
        let Actor(ctx, aggregate) = self;
        let mut aggregate = aggregate.borrow_mut();
        // let mut aggregate = self.aggregate.borrow_mut();
        // let mut aggregate = self.1.borrow_mut();
        // info!("Command: {:?}", input);
        let args = format_args!("");
        ctx.dispatch(LoggingEvents::Trace(Data::new(args)));
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

// impl<'a, T, TContext, TCommands, TEvents, TErrors> Sink
//     // for System<TContext, AggregateSystem<'a, T, TContext, TCommands, TEvents, TErrors>>
//     for System<TContext, AggregateSystem<'a, T, TContext>>
// where
//     TCommands: fmt::Debug,
//     TEvents: fmt::Debug,
//     TErrors: fmt::Debug,
//     TContext: Dispatcher<LoggingEvents<'a>, ()>,
//     T: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>,
// {
//     type TInput = TCommands;
//     type TResult = ();

//     fn send(&self, input: Self::TInput) -> Self::TResult {
//         let mut aggregate = self.aggregate.borrow_mut();
//         // info!("Command: {:?}", input);
//         // self.context.dispatch(LoggingEvents::Trace(format!("Command: {:?}", input)));
//         let result = aggregate.handle(input);
//         match result {
//             Ok(event) => {
//                 // info!("Event: {:?}", event);
//                 // self.context.dispatch(LoggingEvents::Trace(format!("Event: {:?}", event)));
//                 aggregate.update(event);
//             }
//             Err(err) => {
//                 // error!("Error: {:?}", err);
//                 // self.context.dispatch(LoggingEvents::Trace(format!("Error: {:?}", err)));
//             }
//         }
//         ()
//     }
// }

// impl<'a, TContext, TSystem> SystemDef<TContext, TSystem> for LifetimeWrapper<'a, TSystem>
// where
//     TContext: Dispatcher<LoggingEvents<'a>, ()>,
//     TSystem: AggregateRoot + Sized,//AggregateSystem<'a, Self::TContext>
// {
//     fn bind(ctx: TContext) -> AggregateSystem<'a, TContext, Self> {
//         ( ctx, TSystem::default() )
//     }
// }

// impl<'a, T, TContext, TCommands, TEvents, TErrors> SystemDef
//     for T//AggregateSystem<'a, T, TContext, TCommands, TEvents, TErrors>
// where
//     TContext: Dispatcher<LoggingEvents<'a>, ()>,// Sink<TInput = LoggingEvents, TResult = ()>,
//     T: System<TContext, RefCell<AggregateRoot + Default>>,
// {
//     type TContext = TContext;//Sink<TInput = LoggingEvents, TResult = ()>;

//     fn bind(context: TContext) -> System<Self::TContext, Self> {
//         ( context
//         , AggregateSystem {
//                 _commands: PhantomData,
//                 _events: PhantomData,
//                 _errors: PhantomData,
//                 _lifetime: PhantomData,
//                 _context: PhantomData,
//                 // context,
//                 aggregate: RefCell::<T>::default(),
//             }
//         )
//     }
// }

// pub struct SystemContext {}

// pub struct AggregateSystem<'a, T, TContext, TCommands, TEvents, TErrors>
// where
//     T: AggregateRoot,
// {
//     _commands: PhantomData<TCommands>,
//     _events: PhantomData<TEvents>,
//     _errors: PhantomData<TErrors>,
//     _lifetime: PhantomData<&'a ()>,
//     _context: PhantomData<TContext>,
//     // context: TContext,
//     aggregate: RefCell<T>,
// }

// impl<'a, T, TContext, TCommands, TEvents, TErrors> SystemDef
//     for AggregateSystem<'a, T, TContext, TCommands, TEvents, TErrors>
// where
//     TContext: Dispatcher<LoggingEvents<'a>, ()>,// Sink<TInput = LoggingEvents, TResult = ()>,
//     T: AggregateRoot + Default,
// {
//     type TContext = TContext;//Sink<TInput = LoggingEvents, TResult = ()>;

//     fn bind(context: TContext) -> System<Self::TContext, Self> {
//         ( context
//         , AggregateSystem {
//                 _commands: PhantomData,
//                 _events: PhantomData,
//                 _errors: PhantomData,
//                 _lifetime: PhantomData,
//                 _context: PhantomData,
//                 // context,
//                 aggregate: RefCell::<T>::default(),
//             }
//         )
//     }
// }

// impl<'a, T, TContext, TCommands, TEvents, TErrors> Sink
//     for System<TContext, AggregateSystem<'a, T, TContext, TCommands, TEvents, TErrors>>
// where
//     TCommands: fmt::Debug,
//     TEvents: fmt::Debug,
//     TErrors: fmt::Debug,
//     TContext: Dispatcher<LoggingEvents<'a>, ()>,
//     T: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>,
// {
//     type TInput = TCommands;
//     type TResult = ();

//     fn send(&self, input: Self::TInput) -> Self::TResult {
//         let mut aggregate = self.aggregate.borrow_mut();
//         // info!("Command: {:?}", input);
//         // self.context.dispatch(LoggingEvents::Trace(format!("Command: {:?}", input)));
//         let result = aggregate.handle(input);
//         match result {
//             Ok(event) => {
//                 // info!("Event: {:?}", event);
//                 // self.context.dispatch(LoggingEvents::Trace(format!("Event: {:?}", event)));
//                 aggregate.update(event);
//             }
//             Err(err) => {
//                 // error!("Error: {:?}", err);
//                 // self.context.dispatch(LoggingEvents::Trace(format!("Error: {:?}", err)));
//             }
//         }
//         ()
//     }
// }

// impl<'a, T, TContext, TCommands, TEvents, TErrors> Sink
//     for AggregateSystem<'a, T, TContext, TCommands, TEvents, TErrors>
// where
//     TCommands: fmt::Debug,
//     TEvents: fmt::Debug,
//     TErrors: fmt::Debug,
//     TContext: Dispatcher<LoggingEvents<'a>, ()>,
//     T: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors>,
// {
//     type TInput = TCommands;
//     type TResult = ();

//     fn send(&self, input: Self::TInput) -> Self::TResult {
//         let mut aggregate = self.aggregate.borrow_mut();
//         // info!("Command: {:?}", input);
//         // self.context.dispatch(LoggingEvents::Trace(format!("Command: {:?}", input)));
//         let result = aggregate.handle(input);
//         match result {
//             Ok(event) => {
//                 // info!("Event: {:?}", event);
//                 // self.context.dispatch(LoggingEvents::Trace(format!("Event: {:?}", event)));
//                 aggregate.update(event);
//             }
//             Err(err) => {
//                 // error!("Error: {:?}", err);
//                 // self.context.dispatch(LoggingEvents::Trace(format!("Error: {:?}", err)));
//             }
//         }
//         ()
//     }
// }

// pub trait IntoSystem<T, TCommands, TEvents, TErrors>
// where
//     T: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors> + Default,
// {
//     fn bind<'a, TContext>(
//         context: TContext,
//     ) -> System<TContext, AggregateSystem<'a, T, TContext, TCommands, TEvents, TErrors>>
//     where
//         TContext: Sink<TInput = LoggingEvents<'a>, TResult = ()>,
//     {
//         AggregateSystem::bind(context)
//     }
// }

// impl<T, TCommands, TEvents, TErrors> IntoSystem<T, TCommands, TEvents, TErrors> for T where
//     T: AggregateRoot<TCommands = TCommands, TEvents = TEvents, TErrors = TErrors> + Default
// {}
