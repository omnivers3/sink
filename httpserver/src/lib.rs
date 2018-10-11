extern crate sink;
extern crate tiny_http;

// use sink::*;
// use tiny_http;
use self::sink::*;

use std::io::{Error};
use std::marker::{PhantomData};
use std::net::{SocketAddr};

#[derive(Debug)]
pub enum Commands {
    SetValue (u32),
}

#[derive(Clone)]
pub struct ServerConfig {
    addr: SocketAddr,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{0}", self.addr)
    }

    pub fn new(addr: SocketAddr) -> Self {
        ServerConfig {
            addr,
        }
    }
}

//     pub fn new(ip: String, port: String) -> Result<Self, ConfigErrors> {
        
//     }
// }

#[derive(Debug)]
pub struct ServerState {
    count: u32,
    errors: Vec<ServerErrors>,
}

#[derive(Debug)]
pub enum LoggingEvents {
    Error (String),
    Info (String),
    Warning (String),
}

#[derive(Debug)]
pub enum ServerEvents<TCommand> {
    CommandReceived (TCommand),
    LogEmitted (LoggingEvents),
}

#[derive(Debug)]
pub enum ServerErrors {
    CommandHandler,
    FailedToBind (SocketAddr),
    IoError (Error),
    // IoError (std::io::Error),
}

pub struct Server<TInput, TOutput> {
    _input: PhantomData<TInput>,
    _output: PhantomData<TOutput>,
}

fn parse_request(state: &ServerState, _request: &mut tiny_http::Request) -> Result<Commands, ()> {
    Ok (Commands::SetValue(state.count))
}

impl<TOutput> IService for Server<ServerConfig, TOutput>
where
    TOutput: ISink<TInput=ServerEvents<Commands>, TResult=Result<u32, ()>>,
{
    type TInput = ServerConfig;
    type TOutput = TOutput;
    type THandle = Result<ServerState, ServerErrors>;

    fn run(ctx: Self::TInput, tx: Self::TOutput) -> Self::THandle {
        let address = ctx.address();
        let mut state = ServerState {
            count: 0,
            errors: Vec::new(),
        };
        // info!("\nStarting server @ {:?}\n- {:?}", address, state);
        
        tiny_http::Server::http(address)
            .map(|server| {
                for mut request in server.incoming_requests() {
                    state.count += 1;
                    // TODO: Map request into a Command
                    let result = parse_request(&state, &mut request)
                        .map(ServerEvents::CommandReceived)
                        .and_then(|cmd| tx.handle(cmd))
                        // .map(|cmd| tx.handle(cmd))
                    // let result = tx
                    //     .handle(
                    //         ServerEvents::CommandReceived(
                    //             Commands::SetValue(state.count)
                    //         )
                    //     )
                        .map(|result| format!("{0}", result))
                        .map(tiny_http::Response::from_string)
                        .map_err(|_| ServerErrors::CommandHandler)
                        .and_then(|response| {
                            request
                                .respond(response)
                                .map_err(ServerErrors::IoError)
                        });
                    if let Err (err) = result {
                        state.errors.push(err);
                    }
                        
                    // {
                    //     state.errors.append(err);
                    // }
                    // let send_result = tx.handle(Commands::SetValue(state.count));
                    // let response = format!("Request[{:?}]: [{:?}] with result [{:?}]", state.count, request, send_result);
                    // let response = tiny_http::Response::from_string(response);
                    // match request.respond(response) {
                    //     Ok (_) => {},
                    //     Err (err) => return err;
                    // }
                    // let result: Result<(), std::io::Error> = request.respond(response);//.unwrap();
                    // result.unwrap();
                }
                state
            })
            .map_err(|_| ServerErrors::FailedToBind (ctx.addr))
    }
}

// #![feature(custom_attribute)]
// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
// extern crate rocket;

// extern crate sink;

// use sink::*;

// pub struct HttpServer {}

// pub struct HttpRequest {

// }

// pub type ETag = u32;

// pub enum HttpErrors {
//     NotFound,
// }

// impl ISink for HttpServer {
//     type TInput = HttpRequest;
//     type TResult = Result<ETag, HttpErrors>;

//     fn handle(&self, input: HttpRequest) -> Result<ETag, HttpErrors> {
//         // Do parsing, validation, etc here
//         Ok (32)
//     }
// }

// pub struct HttpSession {

// }

// pub enum Commands {}

// impl<TSink> ISource for TSink//HttpServer
// where
//     TSink: ISink<TInput = Self::TOutput, TResult = Result<ETag, HttpErrors>>,
// {
//     type TOutput = Commands;
//     // type THandle = ();

//     // fn bind(self, sink: TSink) -> () {}
// }


// #[get("/<name>/<age>")]
// fn hello(name: String, age: u8) -> String {
//     format!("Hello, {} year old named {}!", age, name)
// }

// fn main() {
//     rocket::ignite().mount("/hello", routes![hello]).launch();
// }