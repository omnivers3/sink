use std::env;
use std::sync::Once;

use super::IConfigProvider;
use env_logger;

static ENV_PROVIDER_INITIALIZE: Once = Once::new();
static RUST_LOG: &'static str = "RUST_LOG";
static DEFAULT_LOG_LEVEL: &'static str = "trace";

fn init_log() {
    ENV_PROVIDER_INITIALIZE.call_once(|| {
        if env::var(RUST_LOG).is_err() {
            env::set_var(RUST_LOG, DEFAULT_LOG_LEVEL);
        }
        env_logger::init();
    });
}

pub struct EnvConfigProvider {}

impl EnvConfigProvider {
    pub fn new() -> Self {
        init_log();
        EnvConfigProvider {}
    }
}

impl IConfigProvider for EnvConfigProvider {
    fn get(&mut self, key: &'static str) -> Option<String> {
        env::var(key).ok()
    }
}

// pub type EnvValue = String;

// pub struct CachedEnvConfigProvider {
//     map: HashMap<&'static str, EnvValue>,
// }

// impl CachedEnvConfigProvider {
//     pub fn new() -> Self {
//         init_log();
//         let map = HashMap::new();
//         CachedEnvConfigProvider {
//             map,
//         }
//     }
// }

// impl IConfigProvider<EnvValue> for CachedEnvConfigProvider {
//     fn get(&mut self, key: &'static str) -> Option<EnvValue> {
//         if !self.map.contains_key(key) {
//             if let Ok(value) = env::var(key) {
//                 self.map.insert(key, value);
//             }
//         }
//         self.map.get(key).map(|v| v.to_owned())
//     }
// }
