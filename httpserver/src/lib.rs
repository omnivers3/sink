extern crate sink;
extern crate tiny_http;

use self::sink::*;

use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;


#[derive(Debug)]
pub enum ServerError {
    AddrParseError(AddrParseError),
    IoError(io::Error),
    PortParseError(ParseIntError),
    // TcpListenerSink (tcp_listener::SinkErrors),
    UnexpectedErr(&'static str),
}

#[derive(Debug)]
pub enum Commands {
    SetValue(u32),
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
        ServerConfig { addr }
    }
}

#[derive(Debug)]
pub struct ServerState {
    count: u32,
    errors: Vec<ServerErrors>,
}

#[derive(Debug)]
pub enum LoggingEvents {
    Error(String),
    Info(String),
    Warning(String),
}

#[derive(Debug)]
pub enum ServerEvents<TCommand> {
    CommandReceived(TCommand),
    LogEmitted(LoggingEvents),
}

#[derive(Debug)]
pub enum ServerErrors {
    CommandHandler,
    FailedToBind(SocketAddr),
    IoError(io::Error),
}

pub struct Server<TInput, TOutput> {
    _input: PhantomData<TInput>,
    _output: PhantomData<TOutput>,
}

fn parse_request(state: &ServerState, _request: &mut tiny_http::Request) -> Result<Commands, ()> {
    Ok(Commands::SetValue(state.count))
}

impl<TOutput> IService for Server<ServerConfig, TOutput>
where
    TOutput: ISink<TInput = ServerEvents<Commands>, TResult = Result<u32, ()>>,
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
                        .and_then(|cmd| tx.send(cmd))
                        .map(|result| format!("{0}", result))
                        .map(tiny_http::Response::from_string)
                        .map_err(|_| ServerErrors::CommandHandler)
                        .and_then(|response| {
                            request.respond(response).map_err(ServerErrors::IoError)
                        });
                    if let Err(err) = result {
                        state.errors.push(err);
                    }
                }
                state
            }).map_err(|_| ServerErrors::FailedToBind(ctx.addr))
    }
}
