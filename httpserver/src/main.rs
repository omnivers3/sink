#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sink;

mod lib;

use std::env;
use std::net::{AddrParseError, SocketAddr};
use std::num::{ParseIntError};
use std::sync::Once;

use sink::*;

use lib::{ Commands, LoggingEvents, Server, ServerConfig, ServerEvents };

static ENV_PROVIDER_INITIALIZE: Once = Once::new();

fn env_or<TKey, TDefault>(key: TKey, default: TDefault) -> String
where
    TKey: Into<String>,
    TDefault: Into<String>,
{
    env::var(key.into()).unwrap_or(default.into())
}

#[derive(Debug)]
pub enum SocketAddrParseError {
    AddrParseError (AddrParseError),
    PortParseError (ParseIntError),
    HostAddressInUse (SocketAddr),
}

fn parse_ipaddr(ip: String, port: String) -> Result<SocketAddr, SocketAddrParseError> {
    // TODO: Validate ability to bind on ip/port?
    ip.parse()
        .map_err(SocketAddrParseError::AddrParseError)
        .and_then(|ip| {
            port.parse()
                .map_err(SocketAddrParseError::PortParseError)
                .map(|port| {
                    SocketAddr::new(ip, port)
                })
        })
}

pub struct EnvProvider {
    server_config: ServerConfig,
}

impl EnvProvider {
    pub fn new() -> Result<Self, SocketAddrParseError> {
        ENV_PROVIDER_INITIALIZE.call_once(|| {
            if env::var("RUST_LOG").is_err() {
                env::set_var("RUST_LOG", "debug");
            }
            env_logger::init();
        });
        parse_ipaddr(
            env_or("HOST_ADDR", "0.0.0.0"),
            env_or("HOST_PORT", "8080")
        )
            .map(ServerConfig::new)
            .map(|server_config| {
                EnvProvider {
                    server_config,
                }
            })
    }
}

pub struct CommandSink {}

impl ISink for CommandSink {
    type TInput = Commands;
    type TResult = Result<u32, ()>;

    fn send(&self, input: Self::TInput) -> Self::TResult {
        info!("Got command: {:?}", input);
        match input {
            Commands::SetValue (value) => {
                Ok (value)
            }
        }
    }
}

pub struct ServerSink {}

impl ISink for ServerSink {
    type TInput = ServerEvents<Commands>;
    type TResult = Result<u32, ()>;

    fn send(&self, input: Self::TInput) -> Self::TResult {
        info!("Got server event: {:?}", input);
        match input {
            ServerEvents::CommandReceived(command) => {
                info!("Got command: {:?}", command);
                Ok (42)
            },
            ServerEvents::LogEmitted(log) => {
                match log {
                    LoggingEvents::Error (msg) => error!("{}", msg),
                    LoggingEvents::Info (msg) => info!("{}", msg),
                    LoggingEvents::Warning (msg) => warn!("{}", msg),
                };
                Ok (0)
            }
        }
    }
}

fn main() {
    match EnvProvider::new() {
        Ok (provider) => {
            match Server::run(provider.server_config, ServerSink{}) {
                Ok (result) => info!("Server terminated successfully: {:?}", result),
                Err (err) => error!("Server runtime error: {:?}", err),
            }
        },
        Err (err) => {
            error!("Server binding error: {:?}", err)
        }
    }
    
}
