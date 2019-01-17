/// The ISink trait aims to provide an abstraction for a thing which can receive values
/// and return the result of each event's receipt along with a handle to, potentially,
/// observe outcome, status and/or value per the Sink type.
///
/// As a base primitive this should enable a message oriented variant of the inbound
/// params to the familiar imperitive Result 'and_then' composition pattern.
///
/// Immediately responding to handle with TResult enables implementations to represent
/// the potential for failure and encapsulate both sync and async processing.
pub trait Sink {
    type TInput;
    type TResult;

    /// `send` accepts an item returning either a result, potentially unit, to the
    /// sender.  In practice the TResult can itself represent a more complex concept
    /// such as a Result<T,E>, a process handle or array index.
    fn send(&self, input: Self::TInput) -> Self::TResult;
}

// pub type UnitSink<TInput> = Sink<TInput=TInput, TResult=()>;

// pub trait UnitSink {
//     type TInput;

//     fn send(&self, input: Self::TInput);
// }

// impl<T, TInput> UnitSink for T
// where
//     T: Sink<TInput=TInput, TResult=()>,
// {
//     type TInput = TInput;
//     // type TResult = ();

//     fn send(&self, input: Self::TInput) {
//         self.send(input)
//         // (self as &Sink<TInput=Self::TInput, TResult=()>).send(input)
//     }
// }

// impl<TInput> Sink for UnitSink<TInput=TInput> {
//     type TInput = TInput;
//     type TResult = ();

//     fn send(&self, input: Self::TInput) -> Self::TResult {
//         (self as &UnitSink<TInput=Self::TInput>).send(input)
//     }
// }

pub trait Dispatcher<TInput> {
    fn dispatch(&self, TInput);
}

impl<TSink, TInput> Dispatcher<TInput> for TSink
where
    TSink: Sink<TInput = TInput, TResult = ()>,
{
    fn dispatch(&self, input: TInput) {
        self.send(input)
    }
}

// pub trait System {
//     type TInput;
//     type TResult;
//     type TOutput;

//     fn send(&self, input: Self::TInput) -> Self::TResult;
// }

pub struct System {
    
}

impl System {
    pub fn run(self) {}
}

pub trait IntoSystem {
    type TInput;
    type TOutput;

    fn into_system(self, _ctx: impl Sink<TInput=Self::TOutput, TResult=()>) -> System
    where
        Self: Sized,
    {
        System {}
    }
}

// pub trait Lift<TInput, TResult> {
//     fn lift<UInput, UResult>(&self, target: Sink<TInput=UInput, TResult=UResult>) -> Sink<TInput=TInput, TResult=UResult>
//     where
//         UInput: TResult,
//     {
//         target.map(self.send)
//     }
// }

pub trait Source {
    type TOutput;

    fn next(&self) -> Self::TOutput;
}

pub struct UnitSource {}

impl UnitSource {
    pub fn new() -> Self {
        UnitSource {}
    }
}

impl Source for UnitSource {
    type TOutput = ();

    fn next(&self) -> Self::TOutput {
        ()
    }
}

// Enables a type to be defaulted and then overridden if mutable
pub trait Initializable: Default {
    type TState;

    fn init(state: Self::TState) -> Self {
        let mut default = Self::default();
        default.apply_state(state);
        default
    }

    fn apply_state(&mut self, state: Self::TState);
}
