use sink::{ Sink, Initializable };

pub trait Aggregate {
    type TCommands;
    type TEvents;
    type TErrors;

    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors>;
    fn apply_event(&mut self, event: Self::TEvents);
}

impl<TCommands, TEvents, TErrors> Sink for Aggregate<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors> {
    type TInput = TCommands;
    type TResult = Result<TEvents, TErrors>;

    fn send(&self, input: Self::TInput) -> Self::TResult {
        self.handle(input)
    }
}

#[derive(Clone, Debug)]
pub struct RegistrationData<'a> {
    name: &'a str,
}

#[derive(Clone, Debug)]
pub enum Commands<'a> {
    Register (RegistrationData<'a>),
    Disable,
    ReEnable,
}

#[derive(Clone, Debug)]
pub enum Events<'a> {
    Registered (RegistrationData<'a>),
    Disabled,
    ReEnabled,
}

#[derive(Clone, Debug)]
pub enum Errors {
    AlreadyRegistered,
    AlreadyDisabled,
    AlreadyEnabled,
    NotInitialized,
}

#[derive(Clone, Debug)]
pub struct State<'a> {
    enabled: bool,
    name: &'a str,
}

#[derive(Clone, Debug)]
pub struct Component<'a> {
    enabled: bool,
    name: &'a str,
}

impl<'a> Default for Component<'a> {
    fn default() -> Self {
        Component {
            enabled: false,
            name: "",
        }
    }
}

impl<'a> Initializable for Component<'a> {
    type TState = State<'a>;

    fn apply_state(&mut self, state: Self::TState) {
        self.enabled = state.enabled;
        self.name = state.name;
    }
}

// pub struct AggregateSystem<TContext, TSystem> {
//     context: TContext,
//     mut system: TSystem,
// }

// impl<TContext, TSystem> AggregateSystem<TContext, TSystem> {
//     pub fn context(&self) -> &TContext {
//         &self.context
//     }

//     pub fn system(&self) -> &mut TSystem {
//         &self.system
//     }
// }

// pub struct Loader<TAggregate> {}

// impl< Loader {
//     pub fn load() -> 
// }

impl<'a> Aggregate for Component<'a> {
    type TCommands = Commands<'a>;
    type TEvents = Events<'a>;
    type TErrors = Errors;

    fn handle(&self, command: Self::TCommands) -> Result<Self::TEvents, Self::TErrors> {
        match (self.name, command) {
            ("", Commands::Register (data)) => Ok(Events::Registered (data)),
            ("", _) => Err(Errors::NotInitialized),
            (_, Commands::Register (_)) => Err(Errors::AlreadyRegistered),
            (_, Commands::Disable) => {
                if self.enabled {
                    Ok (Events::Disabled)
                } else {
                    Err (Errors::AlreadyDisabled)
                }
            },
            (_, Commands::ReEnable) => {
                if self.enabled {
                    Err (Errors::AlreadyEnabled)
                } else {
                    Ok (Events::ReEnabled)
                }
            }
        }
    }

    fn apply_event(&mut self, event: Self::TEvents) {
        match event {
            Events::Registered (data) => {
                self.enabled = true;
                self.name = data.name;
            },
            Events::Disabled => {
                self.enabled = false;
            },
            Events::ReEnabled => {
                self.enabled = true;
            },
        }
    }
}

// impl SystemDef<TContext, TSystem> for TSystem
// where
//     TContext: Dispatcher<Events, ()> + Dispatcher
// {

// }

// pub struct InProcessAggregateSingleton {
//     aggregate: 
// }

// impl<'a, TAggregate> Sink for &'a mut TAggregate {
//     type TInput = TEvent;
//     type TResult = ();

//     fn send(&self, input: )
// }

#[cfg(test)]
mod should {
    use super::*;

    use logging::{ Logging, LoggingEvents };

    #[test]
    fn init_singleton_actor() {
        // env::EnvConfigProvider::new();

        // Component::bind(ctx!{
        //     logging: LoggingEvents | () = logging_sink,
        //     events: Events | () = event_sink,
        //     errors: Errors | () = error_sink,
        //     commands: () | Commands = command_source,
        // })

        let logging_sink = Logging::new();
        let event_vec = VecSink::new();
        let event_sink = &event_vec.map(|event| format!("{:?}", event)).map_result(|_| ());
        let error_sink = FnSink::new(|error: server::Errors| {
            println!("Error Sink: {:?}", error);
        });
        let ctx = ctx!{
            logging: LoggingEvents | () = logging_sink,
            events: Events | () = event_sink,
            errors: Errors | () = error_sink,
            commands: () | Commands = command_source,
        };
        let ping = AggregateActor::<Component>::bind(ctx, "ping");
        let pong = AggregateActor::<Component>::bind(ctx, "pong");
        
        let system = MemorySystem::Run([ping, pong]);


        let mut actor = Component::default();
        println!("Actor1: {:?}", actor);
        let result = actor.handle(Commands::Register (RegistrationData { name: "foo" }));
        println!("Result 1: {:?}", result);
        result.and_then(|event| {
            actor.apply_event(event);
            println!("Actor1: {:?}", actor);
            let result = actor.handle(Commands::Disable);
            println!("Result 2: {:?}", result);
            result
        }).and_then(|event| {
            actor.apply_event(event);
            println!("Actor1: {:?}", actor);
            let result = actor.handle(Commands::ReEnable);
            println!("Result 3: {:?}", result);
            result
        });

        assert!(false);
        // let logging = Logging::new();

        // use product::*;

        // let system = Component::bind(context!{
        //     logging: LoggingEvents | () = logging,
        // });

        // system.send(Commands::Register (RegistrationData { name: "foo" }));
        // system.send(Commands::Disable);
        // system.send(Commands::ReEnable);
    }
}