#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sink;
extern crate tiny_http;

use std::env;
use std::net::{AddrParseError, SocketAddr};
use std::num::{ParseIntError};
use std::sync::Once;

use sink::*;

static ENV_PROVIDER_INITIALIZE: Once = Once::new();

pub enum Commands {
    SetValue (u32),
}

#[derive(Debug)]
pub enum ConfigErrors {
    AddrParseError (AddrParseError),
    PortParseError (ParseIntError),
    InvalidHostAddress (SocketAddr, String),
}

#[derive(Clone)]
pub struct ServerConfig {
    addr: SocketAddr,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{0}", self.addr)
    }

    pub fn new(ip: String, port: String) -> Result<Self, ConfigErrors> {
        // // TODO: Validate ability to bind on ip/port?
        ip.parse()
            .map_err(ConfigErrors::AddrParseError)
            .and_then(|ip| {
                port.parse()
                    .map_err(ConfigErrors::PortParseError)
                    .map(|port| {
                        ServerConfig {
                            addr: SocketAddr::new(ip, port),
                        }
                    })
            })
    }
}

#[derive(Debug)]
pub enum ServerErrors {
    FailedToBind (SocketAddr),
}

pub struct Server {}

impl ISystem for Server {
    type TInput = ServerConfig;
    type TOutput = Commands;
    type TResult = Result<u32, ()>;
    type THandle = Result<u32, ServerErrors>;

    fn bind(ctx: impl IContext<Self::TInput, Self::TOutput, Self::TResult>) -> Self::THandle {
        let config = ctx.next();
        let address = config.address();
        info!("Starting server @ {:?}", address);

        let mut request_index = 0;
        let server = tiny_http::Server::http(address);

        server
            .map(|server| {
                info!("Server running...");
                for mut request in server.incoming_requests() {
                    request_index += 1;
                    info!("Got Request Index: [{:?}]:\t{:?}\n", request_index, request);

                    let send_result = ctx.handle(Commands::SetValue(request_index));

                    let response = format!("Request Index: [{:?}] with result [{:?}]", request_index, send_result);

                    let response = tiny_http::Response::from_string(response);

                    request.respond(response).unwrap();
                }
                request_index
            })
            .map_err(|_| ServerErrors::FailedToBind (config.addr))
    }
}

fn env_or<TKey, TDefault>(key: TKey, default: TDefault) -> String
where
    TKey: Into<String>,
    TDefault: Into<String>,
{
    env::var(key.into()).unwrap_or(default.into())
}

pub struct EnvProvider {
    server_config: ServerConfig,
}

impl EnvProvider {
    pub fn new() -> Result<Self, ConfigErrors> {
        ENV_PROVIDER_INITIALIZE.call_once(|| {
            env_logger::init();
        });
        ServerConfig::new(
            env_or("HOST_ADDR", "0.0.0.0"),
            env_or("HOST_PORT", "8080")
        )
            .map(|server_config| {
                EnvProvider {
                    server_config
                }
            })
    }
}

impl ISource for EnvProvider {
    type TOutput = ServerConfig;

    fn next(&self) -> Self::TOutput {
        self.server_config.to_owned()
    }
}

impl ISink for EnvProvider {
    type TInput = Commands;
    type TResult = Result<u32, ()>;

    fn handle(&self, input: Self::TInput) -> Self::TResult {
        match input {
            Commands::SetValue (value) => Ok (value)
        }
    }
}

fn main() {
    match EnvProvider::new() {
        Ok (ctx) => {
            match Server::bind(ctx) {
                Ok (result) => info!("Server terminated successfully: {:?}", result),
                Err (err) => error!("Server runtime error: {:?}", err),
            }
        },
        Err (err) => {
            error!("Server binding error: {:?}", err)
        }
    }
    
}