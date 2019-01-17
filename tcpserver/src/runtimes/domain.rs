use omnivers3_systems_actor::IntoActorSystem;
use sink::{ Sink, Sink2 };
use sink::fnsink::{ FnSink };
use stdio::*;
use std::cell::{ RefCell };

pub struct SinkSystem<TSignal, TResult, TSink>
where
    TSink: Sink<TInput=TSignal, TResult=TResult>,
{
    _inner: TSink,
}

impl<TSignal, TResult, TSink> SinkSystem<TSignal, TResult, TSink>
where
    TSink: Sink<TInput=TSignal, TResult=TResult>,
{
    pub fn new(inner: TSink) -> SinkSystem<TSignal, TResult, TSink> {
        SinkSystem {
            _inner: inner,
        }
    }
}

pub fn main() {

    let command_counter = RefCell::new(0);
    let _commands = FnSink::new(|e: StdinCommands| {
        let mut counter = command_counter.borrow_mut();
        *counter += 1;
        println!("Command\t[{}]: {:?}", *counter, e);
    });

    let event_counter = RefCell::new(0);
    let events = FnSink::new(|e: StdinEvents| {
        let mut counter = event_counter.borrow_mut();
        *counter += 1;
        println!("Event\t[{}]: {:?}", *counter, e);
    });

    let error_counter = RefCell::new(0);
    let errors = FnSink::new(|err: StdinErrors| {
        let mut counter = error_counter.borrow_mut();
        *counter += 1;
        println!("Errors\t[{}]: {:?}", *counter, err);
    });

    let _unit_sink = FnSink::new(|_item: ()| {
        println!("Unit Sink");
    });

    // let reader = linereader::Config::new();

    let reader = mocklinereader::Config::new(&[
        "product/register {\"name\": \"hammer\"}",
        "product/disable ",
    ]);

    let system1 = reader.bind(&events, &errors);

    let unit_errors = FnSink::new(|_unit: ()| {
        println!("Unit");
    });

    // // let root = console::Config::new().bind((&system1, &unit_errors));
    let root = console::Config::new().bind(&system1, &unit_errors);

    // let root = root
    //     .and_then(&system1) // map ok branch into target
    //     .map(| in_event: StdinEvents | {
    //         println!("In Event: {:?}", in_event);
    //         ()
    //     })
    //     .bind(&unit_sink, &unit_sink);
    //     // TODO: .run(); // tranlates to .bind(&unit_sink, &unit_sink)

    // let System2(system1_a, system1_b) = system1.tee(); // New actors which replicate
    // let system1_b = system1_b.skip(1); // New actor which only emits after # more have passed
    // let system1 = system1_a.zip(system1_b); // New actor which listens to both inputs and emits tuples


    
    root.send(());

    let temp = Sink2::new(&system1, &unit_errors);

    let Sink2(sys, unit) = temp;//.spread();

    sys.send(StdinCommands::Await);
    unit.send(());
    
    // let system1 = uc.into_inner();
    // let actor2 = linereader::Config::new();
    // actor1.run();

    // let system1 = ActorSystem::new(actor1, &events, &errors);
    // let system2 = ActorSystem::new(actor2, &events, &errors);

    // let console = ActorSystem::new(console::Config::new(), &system1, &unit_errors);

    // let system = {
    //     use 
    //     use mocklinereader;
    // }
    // runtime::from(vec![
    // system1
    // system1.lift(|signal| {

    // })
    use stdio::StdinCommands::*;

    // console.send(());

    // &system1.send(Await);
    // &system1.send(Initialize);
    let system1 = &system1.clone();

    system1.send(Initialize);
    system1.send(Await);
    system1.send(Await);
    // system1.send(Initialize);
    // system2.send(Initialize);
    // system1.send(Await);
    // system1.send(Await);
    // system2.send(Await);
    // system1.send(Await);
    // system2.send(Initialize);
    // system2.send(Await);

}


// pub struct ActorRef {

// }

// pub trait ActorSys {
//     fn run(&self, msg: &str);
// }

// mod temp {
//     use super::{ ActorSys };
//     use component::{ Actor };
    
//     impl<T> ActorSys for T
//     where
//         T: Actor,
//     {
//         fn run(&self, msg: &str) {
//             println!("RUN {:?}", msg);
//         }
//     }
// }

// pub mod a {
//     use self::temp::*;

//     linereader::Config::new()
// }


// mod runtimes {
//     use super::{ Runtime };

//     pub struct InProcess {}

//     impl Runtime for InProcess {
//         type TResult = ();

//         fn run(self, system: impl System<TSignal=()>) -> Self::TResult {
//             loop { // Unit System is like the main() / event loop
//                 system.send(());
//             }
//         }
//     }
// }

// pub trait Runtime {
//     type TResult;

//     // fn run(self, system: impl System<TSignal=()>) -> Self::TResult;
//     fn run(self) -> Self::TResult;
// }

// impl<'a, TSignal, TResult> System<'a, TSignal, TResult> {
//     pub fn handle(&self, signal: TSignal) -> TResult {
//         // Start with a simple pass through
//         (self.inner)(signal)
//     }
// }

// impl<TSignal, TResult> Sink for System<TSignal, TResult> {
//     type TInput = TSignal;
//     type TResult = TResult;

//     fn send(&self, input: Self::TInput) -> Self::TResult {
//         self.handle(input)
//     }
// }

// impl<TResult, TSystem> Runtime for TSystem
// where
//     TSystem: Sink<TInput=(), TResult=TResult>,
// {
//     type TResult = TResult;

//     fn run(self) -> Self::TResult {
//         self.send(())
//     }
// }

// pub trait IntoSystem<TSignal, TResult> {
//     fn into() -> System<TSignal, TResult>;
// }

// pub struct PrintlnSystem {
//     message: String,
// }

// impl PrintlnSystem {
//     pub fn new(message: String) -> Self {
//         PrintlnSystem {
//             message,
//         }
//     }
// }

// impl IntoSystem<(), ()> for PrintlnSystem {
//     type TSignal = ();
//     // type TResult = ();
// }


// #[derive(Debug)]
// pub struct GenericSystem {}

// impl System for GenericSystem {}

// pub trait IntoSystem {
//     fn into_system<T>(self) -> T where T: System;
// }

// impl<TCommands, TEvents, TErrors, TResult, TActor> From<TActor> for GenericSystem
// where
//     TActor: Actor<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors, TResult=TResult>,
// {
//     fn from(actor: TActor) -> GenericSystem {
//         GenericSystem {}
//     }
// }

// impl<TCommands, TEvents, TErrors, TResult, TActor> From<TActor> for &System
// where
//     TActor: Actor<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors, TResult=TResult>,
//     Self: Sized,
// {
//     fn from(actor: TActor) -> &loip
//     System {
//                   &GenericSystem {}
//     }
// }

// impl<TCommands, TEvents, TErrors, TResult, TActor> IntoSystem for TActor
// where
//     TActor: Actor<TCommands=TCommands, TEvents=TEvents, TErrors=TErrors, TResult=TResult>
// {
//     fn into_system<T>(self) -> T where T: System {
//         GenericSystem {}
//     }
// }


// mod eventstores {
//     pub trait 
//     pub struct HashStore {

//     }

//     impl HashStore {
//         pub fn new(loader: impl FnMut(&[TEvents]) -> TState) -> Self {

//         }
//     }

//     impl System for HashStore {
//         type TSignal = usize;


//     }
// }

// pub trait System<'a, 'b> {
//     fn sink<TInput, TResult>(&'a self) -> &'b Sink<TInput=TInput, TResult=TResult>;
// }

// pub trait BindToSystem<'a> {
//     type TInput;
//     type TResult;

//     fn bind_to(self, system: impl System<'a>);// -> &'a Sink<TInput=Self::TInput, TResult=Self::TResult>;
// }

// impl<'a, T> BindToSystem<'a> for T
// where
//     T: Actor,
// {
//     type TInput = T::TCommands;
//     type TResult = T::TResult;

//     fn bind_to(self, system: impl System<'a>) {// -> &'a Sink<TInput=Self::TInput, TResult=Self::TResult> {
//         println!("Bind actor");
//         // system.sink()
//     }
// }

// pub struct LocalSystem {
//     sinks: Vec<impl Sink<
// }

// impl<'a, 'b> System<'a> for LocalSystem {
//     fn sink<TInput, TResult>(&self) -> &'b Sink<TInput=TInput, TResult=TResult> {
//         // &FnSink::new(|e: TInput| TResult)
//     }
// }

// mod console {
//     // use logging::{ LoggingEvents };
//     // use sink::{ IntoSystem, Sink, System };

//     pub struct ConsoleArgs {
//         values: Vec<String>,
//     }

//     pub enum Signals {
//         BindArgs (ConsoleArgs),
//     }

//     pub enum Events {
//         ArgsReceived (ConsoleArgs),
//     }

//     pub enum Errors {
//         InvalidArgs (String),
//     }

//     pub struct App {}

//     impl App {
//         pub fn new() -> Self {
//             App {}
//         }
//     }

//     impl Default for App {
//         fn default() -> Self {
//             // Load args
//             // Make app
//             // Send init command as needed
//             // 
//             App {}
//         }
//     }
// }

// pub struct ConsoleApp

    // stdinlinereader::State::default()
    //     .into_system(FnSink::new(|_| {}))
    //     .run();

    // stdinlinereader::State::default()
    // App::<Console>::default()
    // console::App::new()

    // let system = System::new();

    // let sink = FnSink::new(|e| {
    //     println!("E: {:?}", e);
    // });

    // sink.send()

    // system.run();

    // console::App::new()
    //     .into_system(FnSink::new(|_| {}))
    //     .run();
    //stdinlinereader::Actor::echo().run();
// }

    // impl State {
    //     fn handle(&self, command: Commands) -> Events {
    //         use Commands;
    //         match command {
    //             AwaitLine => {
                    
    //             }
    //         }
    //     }

    //     fn apply(&mut self, event: Events) {
    //         use Events;
    //         match event {
    //             AwaitingLine => {
    //                 let io::stdin().lock().read_line(self.buffer)
    //             }
    //         }
    //         if self.stopped {
    //             panic!("Invalid state");
    //         }
    //         if !self.running {
    //             self.running = true;
    //         }
    //         match event {
    //             LineReceived (_) => {},
    //             Listening => self.listening = true,
    //             Stopped => self.stopped = true,
    //         }
    //     }
    // }

    // pub struct SystemWrapper<'a> {
    //     ctx: &'a Sink<TInput=Events, TResult=()>,
    // }

    // // impl<'a> System<'a> {
    // pub trait System {
    //     type TEvents;

    //     fn new(ctx: &'a Sink<TInput=Self::Events, TResult=()>) -> Self {
    //         SystemWrapper {
    //             ctx,
    //         }
    //     }

    //     fn run() {
    //         // loop
    //         // get
    //         // get mut state
    //         // 
    //     }
    // }
// }
// pub trait RefSource<'a> {
//     type TInput;
//     type TResult;
//     type TOutput;

//     fn sink(&self) -> &'a Sink<TInput=Self::TInput, TResult=Self::TResult>;
//     fn source(&self) -> &'a Source<TOutput=Self::TOutput>;
//     fn duplex(&self) -> (&'a Source<TOutput=Self::TOutput>, &'a Sink<TInput=Self::TInput, TResult=Self::TResult>) {
//         (self.source(), self.sink())
//     }
// }

// fn apply<A, B, C, G>(mut f: impl FnMut(B) -> G, a: A)
// -> impl FnMut(&B) -> C
//         where  
//              G: FnMut(A) -> C,
//              B: Copy,
//              A: Clone {
//     move |b| f(*b)(a.clone())
// }

// pub enum StdoutCommands {
//     WriteLine (String),
// }

// pub fn parse_stdin(input: StdinEvents) -> Option<StdoutCommands> {
//     use self::StdoutCommands::*;
//     match input {
//         StdinEvents::LineReceived (line) => {
//             if line.len() % 10 == 0 {
//                 None
//             } else if line.len() % 2 == 0 {
//                 Some (WriteLine(line))
//             } else {
//                 Some (WriteLine(format!("foo: {}", line)))
//             }
//         }
//         _ => None
//     }
// }

// pub struct ConsoleApp {
//     running: bool,
// }

// impl ConsoleApp {
//     pub fn new() -> Self {
//         ConsoleApp {
//             running: false,
//         }
//     }
// }

// impl ConsoleApp {
//     pub fn 
// }

// impl Saga for ConsoleApp {
//     type TEvents = StdinEvents;

//     fn handle(&self, event: Self::TEvents)
// }

// pub struct ContextWrapper<TContext, TInner> {
//     _context: PhantomData<TContext>,
//     _inner: PhantomData<TInner>,
//     // context: TContext,
//     // inner: TInner,
// }

// impl ContextWrapper<TContext, TInner> {
//     pub fn new() -> Self {
//         ContextWrapper {
//             _context: PhantomData,
//             _inner: PhantomData,
//         }
//     }
// }

// pub trait Context<TContext> {
//     fn bind(self, ctx: TContext) -> ContextWrapper<TContext, ConsoleApp> {
//         Wrapper::new()
//     }
// }

// impl<T> Context<T> for ConsoleApp
// where
//     T: Dispatcher<LoggingEvents> + Dispatcher<String>,
// {}

// pub trait ContextSink {
//     type TContext;
//     type TInput;
//     type TResult;

//     fn send(&self, ctx: Self::TContext, input: Self::TInput) -> Self::TResult;
// }

// impl<TContext> ContextSink for ConsoleApp
// where
//     TContext:
//         Dispatcher<LoggingEvents> +
//         Dispatcher<String>,
// {
//     type TContext = TContext;
//     type TInput = String;
//     type TResult = ();

//     fn send(&self, ctx: Self::TContext, input: Self::TInput) -> Self::TResult {
//         ctx.dispatch(trace!("Starting Console App..."));
//         match input {
//             "exit" => ctx.dispatch("Exiting...".to_owned()),
//             input => ctx.dispatch("Unexpected input {:?}", input),
//         }
//     }
// }



// impl Sink for Wrapper {
//     type TInput = String;
//     type TResult = ();

//     fn send(&self, input: Self::TInput) -> Self::TResult {
//         self.inner.handle(self.context, input)
//     }
// }

// impl Sink for ConsoleApp {
//     type TInput = String;
//     type TResult = ();

//     fn send(&self, input: Self::TInput) -> Self::TResult {

//     }
// }

// impl<TContext> Runtime<TContext> for ConsoleApp
// where
//     TContext:
//         Source<TOutput=String> +
//         Dispatcher<LoggingEvents> +
//         Dispatcher<String>,
// {
//     fn run(self, ctx: TContext) {
//         ctx.dispatch(trace!("Starting Console App..."));
//         match ctx.next() {
//             "exit" => ctx.dispatch("Exiting...".to_owned()),
//             input => ctx.dispatch("Unexpected input {:?}", input),
//         }
//     }
// }

// pub fn main() {
//     // Opaque bridge sink into local stdout logging runtime
//     let logging_sink = Logging::new();

//     // logging_sink.send(trace!("Starting Domain Definition"));

//     // let source = Alternate::new();
//     // let source = UnitSource::new();

//     // let app = App::<ConsoleState>::run(ctx! {
//     //     logging: LoggingEvents = logging_sink,
//     // });

//     let runtime = StdinLineReader::new();
//     // let runtime = MockLineReader::new(&["foo", "barz", "fiz", "flop", "fizzlebopz"]);
//     // let sink = StdoutLineWriter::new();

//     // let in_proc = InProcRuntime::new();

//     let ctx = ctx! {
//         logging: LoggingEvents = logging_sink,
//         // events: StdinEvents = app,
//     };

//     // runtime.run(ctx);

//     // let t = logging_ctx.next();
//     // println!("Next: {:?}", t);
//     // let t = logging_ctx.next();
//     // println!("Next: {:?}", t);
//     // let t = logging_ctx.next();
//     // println!("Next: {:?}", t);
//     // let t = logging_ctx.next();
//     // println!("Next: {:?}", t);

    
// }

// Runtime::new()
//     .bind(
//         ctx! {
//             logging: LoggingEvents = logging_sink,//Logging::new(),
//         },
//         StdinLineReader::new(),
//         // MockLineReader::new(&["foo", "barz", "fiz", "flop", "fizzlebopz"]),
//         StdoutLineWriter::new(),
//         |sink| sink
//             .reduce() // skip broken entries
//             .map(parse_stdin)
//             .map_result(|_| ())
//     )
//     .run();

// InProcRuntime::new()
//     .bind(
//         ctx! {
//             logging: LoggingEvents = logging_sink,//Logging::new(),
//         },
//         StdinLineReader::new(),
//         // MockLineReader::new(&["foo", "barz", "fiz", "flop", "fizzlebopz"]),
//         StdoutLineWriter::new(),
//         | sink | sink
//             .reduce() // skip broken entries
//             .map(parse_stdin)
//             .map_result(|_| ())
//     )
//     .run();

// let stdout = in_proc
//     .bind(StdoutLineWriter::new(), logging_ctx);


// stdin.run(| stdin: Sink<TInput | {
//     stdout.run(| stdout | {
//         stdout
//             .reduce() // skip broken entries
//             .map(parse_stdin)
//             .map_result(|_| ())
//             .map(stdin.send)
//     })
// })
// InProcRuntime::new()
//     .bind(source, ctx! {
//         logging: LoggingEvents = Logging::new(),
//     })
//     .run(| stdin | {
//         InProcRuntime::new()
//             .bind(StdoutLineWriter::new(), ctx! {
//                 logging: LoggingEvents = Logging::new(),
//             })
//             .run(| stdout | {
//                 stdout
//                     .reduce() // skip broken entries
//                     .map(parse_stdin)
//                     .map_result(|_| ())
//             })
//     });

// pub struct LocalThreadRuntime {}

// impl LocalThreadRuntime {
//     pub fn new() -> Self {
//         LocalThreadRuntime {}
//     }
// }

// pub struct RemoteHttpRuntime {}

// impl RemoteHttpRuntime {
//     pub fn new() -> Self {
//         RemoteHttpRuntime {}
//     }
// }

// pub struct RemoteGrpcRuntime {}

// impl RemoteGrpcRuntime {
//     pub fn new() -> Self {
//         RemoteGrpcRuntime {}
//     }
// }


// pub fn main2() {
//     // Opaque bridge sink into local stdout logging runtime
//     let logging_sink = Logging::new();

//     logging_sink.send(trace!("Starting Domain Definition"));

//     let local_thread_console = LocalThreadRuntime::new();
//     let remote_http_like_counter = RemoteHttpRuntime::new();
//     let remote_grpc_dislike_counter = RemoteGrpcRuntime::new();

//     // // Build refs to both the source and sink edges of the std console
//     // let (stdin_events_source, stdout_line_sink) = local_thread_console.duplex();

//     // // Build refs to both the source and sink edges of the like service over http
//     // let (like_source, like_sink) = remote_http_like_counter.duplex();

//     // // Build refs to both the source and sink edges of the dislike service over grpc
//     // let (dislike_source, dislike_sink) = remote_grpc_dislike_counter.duplex();

//     let in_proc_console = InProcRuntime::new();

//     // like_counter::Manifest
// }


// mod bounded_contexts {
//     use sink::{ Sink };

//     pub mod like_counter {
//         // use aggregates::counter::*;

//         // pub struct Manifest {}

//         // impl BoundedContextDef for Manifest {

//         // }
//     }

//     pub mod dislike_counter {
//         // use aggregates::counter::*;

//     }

//     // pub struct BoundedContext<'a> {
//     //     display_name: &'a str,

//     // }

//     // pub struct Manifest<TInput> {
//     //     input: TInput,
//     // }

//     pub trait SinkProvider<'a, TInput, TResult> {
//         fn sink(&self) -> &'a Sink<TInput=TInput, TResult=TResult>;
//     }

//     pub trait Manifest {
//         type TInput;
//         type TOutput;
//     }

//     pub mod server {
//         // use super::{ Manifest };
//         // use sink::sink::{ Dispatcher };

//         // pub enum Commands {
//         //     AddLike,
//         //     AddDislike,
//         // }

//         // pub struct Dependencies {}

//         // pub struct Context {}

//         // impl Manifest for Context {
//         //     type TInput = Commands;
//         //     type TOutput = ();
//         // }

//         // impl<TContext> BoundedContext for Manifest
//         // where
//         //     TContext: Dispatcher<Events>,
//         // impl<TContext> BoundedContext for Manifest
//         // where
//         //     TContext: Dispatcher<Events>,
//         //     // TContext: Dispatcher<Events>,
//         // {
//         //     type TContext = TContext;
//         // }
//     }

// }

// // use self::bounded_contexts::*;

// pub struct Alternate {
//     value: RefCell<bool>,
// }

// impl Alternate {
//     pub fn new() -> Self {
//         Alternate {
//             value: RefCell::new(false),
//         }
//     }
// }

// impl Source for Alternate {
//     type TOutput = Option<()>;

//     fn next(&self) -> Self::TOutput {
//         let mut value = self.value.borrow_mut();
//         *value = !*value;
//         if *value {
//             Some(())
//         } else {
//             None
//         }
//     }
// }


// pub struct InProcRuntime {}

// impl InProcRuntime {
//     pub fn new() -> Self {
//         InProcRuntime {}
//     }
// }

// pub trait Runtime {
//     fn bind<TContext, TSource, TSink, FMap>(ctx: TContext, source: TSource, sink: TSink, fmap: FMap) -> ()
//     where
//         T;
// }

// impl Runtime for InProcRuntime {
//     fn bind(ctx: TContext, source: TSource, sink: TSink, fmap: FMap) -> () {

//     }
// }
