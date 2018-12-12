// pub use component::{ Actor, Runtime };
// pub use sink::*;
// pub use sink::{ Dispatcher };

#[derive(Clone, Debug)]
pub enum StdinCommands {
    Initialize,
    Await,
}

#[derive(Clone, Debug)]
pub enum StdinEvents {
    Initialized,
    Waiting,
    LineReceived (usize, String),
}

#[derive(Clone, Debug)]
pub enum StdinErrors {
    AlreadyInitialized,
    NotInitialized,
    IoError (String),
}

pub mod console {
    use component::{ Actor, ActorState };
    use sink::*;

    use stdio::*;
    use stdio::StdinCommands::*;

    #[derive(Debug)]
    pub struct State {
    }

    #[derive(Clone, Debug)]
    pub struct Config {}

    impl Config {
        pub fn new() -> Self {
            Config {}
        }
    }

    impl ActorState<Config> for State {
        fn from(_config: &Config) -> Self {
            State {}
        }
    }

    impl Actor for Config {
        type TState = State;
        type TCommands = ();
        type TEvents = StdinCommands;
        type TErrors = ();
        type TResult = ();

        fn handle(&self,
            _state: &mut Self::TState,
            _command: (),
            events: impl Sink<TInput=Self::TEvents, TResult=()>,
            _errors: impl Sink<TInput=Self::TErrors, TResult=()>
        ) -> Self::TResult {
            events.send(Initialize);
            loop {
                events.send(Await);
            }
        }
    }
}

pub mod linereader {
    use std::io;
    use std::io::{ BufRead };//, Stdin, Stdout, Write };

    use component::{ Actor, ActorState };
    use sink::*;

    use stdio::*;
    use stdio::StdinCommands::*;
    use stdio::StdinEvents::*;
    use stdio::StdinErrors::*;

    #[derive(Debug)]
    pub struct State {
        stdin: Option<io::Stdin>,
    }

    #[derive(Clone, Debug)]
    pub struct Config {}

    impl Config {
        pub fn new() -> Self {
            Config {}
        }
    }

    impl ActorState<Config> for State {
        fn from(_config: &Config) -> Self {
            State {
                stdin: None,
            }
        }
    }

    impl Actor for Config {
        type TState = State;
        type TCommands = StdinCommands;
        type TEvents = StdinEvents;
        type TErrors = StdinErrors;
        type TResult = ();

        fn handle(&self,
            state: &mut Self::TState,
            command: Self::TCommands,
            events: impl Sink<TInput=Self::TEvents, TResult=()>,
            errors: impl Sink<TInput=Self::TErrors, TResult=()>
        ) -> Self::TResult {
            if let Some (ref mut stdin) = state.stdin {
                match command {
                    Initialize => {
                        errors.send(AlreadyInitialized);
                    }
                    Await => {
                        events.send(Waiting);
                        let mut buffer = String::default();
                        match { stdin.lock().read_line(&mut buffer) } {
                            Ok (len) => {
                                events.send(LineReceived (len, buffer));
                            }
                            Err (err) => {
                                errors.send(IoError (format!("{:?}", err)));
                            }
                        }
                    }
                }
            } else {
                match command {
                    Initialize => {
                        state.stdin = Some(io::stdin());
                        events.send(Initialized);
                    }
                    Await => {
                        errors.send(NotInitialized);
                    }
                }
            }
        }
    }
}

pub mod mocklinereader {
    use std::iter::Iterator;
    use std::time::{ Duration };
    use std::thread;

    use component::{ Actor, ActorState };
    use sink::*;

    use stdio::*;
    use stdio::StdinCommands::*;
    use stdio::StdinEvents::*;
    use stdio::StdinErrors::*;

    #[derive(Clone, Debug)]
    pub struct Config {
        values: Vec<String>,
        delay_ms: u32,
    }

    impl Config {
        pub fn new<TSource>(source: TSource) -> Self
        where
            TSource: IntoIterator,
            TSource::Item: ToString,
        {
            let mut values = vec![
                "foo".to_owned(),
                "bar".to_owned(),
                "baz".to_owned(),
            ];
            let mut source: Vec<String> = source
                .into_iter()
                .map(|i| i.to_string())
                .collect();
            values.append(&mut source);
            let values = values.iter().map(|i| format!{"{}\n", i}).collect();
            
            Config {
                values,
                delay_ms: 10,
            }
        }
    }

    pub struct State {
        values: Vec<String>,
        index: usize,
        initialized: bool,
    }

    impl ActorState<Config> for State {
        fn from(config: &Config) -> Self {
            State {
                values: config.values.clone(),
                index: 0,
                initialized: false,
            }
        }
    }

    impl Iterator for State {
        type Item = String;

        fn next(&mut self) -> Option<String> {
            if self.values.len() <= self.index {
                None
            } else {
                let result = self.values[self.index].to_owned();
                self.index += 1;
                Some (result)
            }
        }
    }

    impl Actor for Config {
        type TState = State;
        type TCommands = StdinCommands;
        type TEvents = StdinEvents;
        type TErrors = StdinErrors;
        type TResult = ();

        fn handle(&self,
            state: &mut Self::TState,
            command: Self::TCommands,
            events: impl Sink<TInput=Self::TEvents, TResult=()>,
            errors: impl Sink<TInput=Self::TErrors, TResult=()>
        ) -> Self::TResult {
            match (command, state.initialized) {
                (Initialize, true) => {
                    errors.send(AlreadyInitialized)
                }
                (Initialize, false) => {
                    state.initialized = true;
                    events.send(Initialized);
                }
                (Await, false) => {
                    errors.send(NotInitialized)
                }
                (Await, true) => {
                    events.send(Waiting);
                    thread::sleep(Duration::from_millis(self.delay_ms.into()));
                    if let Some (line) = state.next() {
                        events.send(LineReceived (line.len(), line));
                    } else {
                        thread::park();
                    }
                }
            }
        }
    }
}

// pub mod stdoutlinewriter {

//     pub struct State {
//         stdout: Stdout,
//     }

//     impl StdoutLineWriter {
//         pub fn new() -> Self {
//             StdoutLineWriter {
//                 stdout: io::stdout(),
//             }
//         }
//     }

//     impl Sink for StdoutLineWriter {
//         type TInput = String;
//         type TResult = Result<(), io::Error>;

//         fn send(&self, value: Self::TInput) -> Self::TResult {
//             let mut lock = self.stdout.lock();
//             write!(lock, "{}\n", value)
//         }
//     }

// }