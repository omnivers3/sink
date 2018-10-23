extern crate sink;
extern crate tcp_server;

// use log::*;
// use env::*;
// use net::*;
use sink::*;
use std::cell::RefCell;
use std::marker::PhantomData;
use tcp_server::*;
// use tcp_server::component::IComponent;

// static HOST_ADDR_KEY: &'static str = "HOST_ADDR";
// static HOST_ADDR_DEFAULT: &'static str = "0.0.0.0";
// static HOST_PORT_KEY: &'static str = "HOST_PORT";
// static HOST_PORT_DEFAULT: &'static str = "8080";

#[derive(Debug)]
pub enum AppCommands {
    Foo,
    Bar,
}

pub enum AppErrors {}

pub struct CommandSource {
    queue: RefCell<Vec<AppCommands>>,
}

impl CommandSource {
    pub fn new(queue: Vec<AppCommands>) -> Self {
        let queue = RefCell::new(queue);
        CommandSource { queue }
    }
}

impl ISource for CommandSource {
    type TOutput = Option<AppCommands>;

    fn next(&self) -> Self::TOutput {
        self.queue.borrow_mut().pop()
        // let mut queue: &Vec<AppCommands> = &*self.queue.borrow_mut();
        // queue.pop()
    }
}

pub struct Harness<
    TComponent,
    TCommands,
    TEvents,
    TErrors
>
where
    TComponent: component::IComponent<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors>,
{
    _commands: PhantomData<TCommands>,
    _events: PhantomData<TEvents>,
    _errors: PhantomData<TErrors>,
    component: RefCell<TComponent>,
}

impl<TComponent, TCommands, TEvents, TErrors> Harness<TComponent, TCommands, TEvents, TErrors>
where
    TComponent: component::IComponent<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors>,
{
    pub fn new(component: TComponent) -> Self {
        Harness {
            _commands: PhantomData,
            _events: PhantomData,
            _errors: PhantomData,
            component: RefCell::new(component),
        }
    }
}

impl<TComponent, TCommands, TEvents, TErrors> sink::ISink for Harness<TComponent, TCommands, TEvents, TErrors>
where
    TComponent: component::IComponent<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors>,
{
    type TInput = TCommands;
    type TResult = ();

    fn send(&self, command: TCommands) -> Self::TResult {
        let mut component = self.component.borrow_mut();
        let result = component.handle(command);
        if let Ok (event) = result {
            component.update(event);
        }
        ()
    }
}

fn main() {
    let harness = Harness::new(net::Component::default());
    harness.send(net::Commands::bind_addresses("localhost:8080"));
    harness.send(net::Commands::Accept);
}

// let config = EnvConfigProvider::new();
// let server = Server::new(config);

// let _logging = Logging {};
// let _cmd = CommandSource::new(vec![AppCommands::Foo, AppCommands::Bar]);

// use tcp_listener::Commands::*;
// use tcp_listener::*;

// let runtime: Runtime<Component> = Runtime::new();

// let mut system = net::Component::init(None);
// let result = system
//     .handle(net::Commands::bind_addresses("localhost:8080"))
//     .and_then(|event| {
//         system.update(event);
//         system.handle(net::Commands::Accept)
//             .map(|event| system.update(event))
//     });

// println!("bind result: {:?}", result);

// pub struct App {}

// impl IContext for App {}

// impl<TContext> IService for App<TContext>
// where
//     TContext:
// {

// }

// pub struct App<TCommandSource, TLoggingSink> {
//     _cmd: PhantomData<TCommandSource>,
//     _log: PhantomData<TLoggingSink>,
// }

// impl<TCommandSource, TLoggingSink> App<TCommandSource, TLoggingSink> {
//     pub fn new() -> Self {
//         App {
//             _cmd: PhantomData,
//             _log: PhantomData,
//         }
//     }
// }

// impl<TCommandSource, TLoggingSink> IService for App<TCommandSource, TLoggingSink>
// where
//     TCommandSource: ISource<TOutput=Option<AppCommands>>,
//     TLoggingSink: ISink<TInput=LoggingEvents, TResult=()>,
// {
//     type TContext = (TCommandSource, TLoggingSink);

//     fn bind(self, (cmd, log): Self::TContext) {
//         loop {
//             if let Some(command) = cmd.next() {
//                 log.send(LoggingEvents::Info(format!("Got command {:?}", command)));
//             } else {
//                 break;
//             }
//         }
//     }
// }

// mod string_source {
//     use sink::{ ISink, ISource };

//     // type StringSink = ISink<TInput=String, TResult=()>;

//     impl<T> ISource for T
//     where
//         // T: IContext<T=String>,
//         T: ISink<TInput=String, TResult=()>,
//     {
//         type TOutput = String;

//         fn next(&self) -> Self::TOutput {
//             "asdf".to_owned()
//         }
//     }
// }

// mod app {
//     use sink::{ ISink };

//     // pub struct App {
//     //     // logging
//     // }

//     // impl App {
//     //     pub fn new(
//     //         source: impl ISource<TItem=String>,
//     //         sink: impl ISink<TItem=String>
//     //     ) -> Self {

//     //     }
//     // }

//     // impl App where Self: IContext<T=String> {}

//     impl ISink for App {
//         type TInput = String;
//         type TResult = ();

//         fn send(&self, input: Self::TInput) -> Self::TResult {
//             println!("Input: {}", input);
//             ()
//         }
//     }
// }

// pub struct InProcRuntime<TState, TComponent>
// where
//     TComponent: IComponent<TState=TState, TCommands=TCommands, TEvents=TEvents, TErrors=TErrors>,
// {
//     component: TComponent,
// }

// impl<TState, TCommands, TEvents, TErrors, TComponent> InProcRuntime<TComponent>
// where
//     // TState: Default,
//     TComponent: IComponent<TState=TState, TCommands=TCommands, TEvents=TEvents, TErrors=TErrors>,
// {
//     pub fn default() -> Self {
//         InProcRuntime {
//             component: TComponent::init(None),
//         }
//     }
// }

// match result {
//     Err (_) => { println!("Error"); },
//     Ok (event) => system.borrow_mut().update(event)
// }
// let system = system.update(event);
// let result = system.borrow_mut().handle(BindSocket("localhost:8080"));
// println!("bind result: {:?}", result);
// let result = system.borrow_mut().handle(Accept);
// println!("accept result: {:?}", result);

// system.run(|tx| {
//     tx.send
// });
// let event = system.next(); // events

// println!("System: {:?} - {:?}", system, event);

// let listener = tcp_listener::System::new().send("localhost:8080");

// println!("System: {:?}", listener);

// Server::new()

// Server::new()
//     .bind("localhost:8080", |addr| {
//         SocketContext {}
//     })
//     .and_then(|server| {
//         server.start()
//     })
//     .map(|result| {
//         println!("Server result: {:?}", result);
//     })
//     .map_err(|err| {
//         println!("Server error: {:?}", err);
//     });
// }
// .map(|req| { // Would be tcp packets
//     println!("Server Request: {:?}", req);
//     req
// })
// .start();

// App::new()
// Should fail to compile due to missing source of T

// let server = Server::run((), (logging.to_owned(), logging));

// let service = Service::new();
// let server = Server::new(service);
// let app = App::new();
// app.bind((cmd, logging));
// mod app {
//     // use super::logging;

//     // App::bind();
// }

// }

//     parse_ipaddr(
//         "0.0.0.0".to_owned(),
//         "8080".to_owned()
//     )
//         .map_err(|err| {
//             Errors::ParseError (err)
//         })
//         .and_then(|addr| {
//             server
//                 .send(Commands::Bind(addr))
//                 .map(|event| {
//                     match event {
//                         Events::Listening (listener, addr) => {
//                             println!("Listening.... {:?} - {:?}", listener, addr);
//                         }
//                     }
//                 })
//         })
//         .unwrap();
// }
