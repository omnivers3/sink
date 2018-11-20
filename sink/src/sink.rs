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

// pub trait SinkContainer<'a, TInput, TResult> {
//     fn sink(&'a self) -> &'a Sink<TInput=TInput, TResult=TResult>;
// }

// pub trait IntoSink<TInput, TResult> {
//     fn as_sink<'a>(&'a self) -> &'a Sink<TInput=TInput, TResult=TResult>;
// }

// pub trait Dispatcher<TInput, TResult> {
//     fn dispatch(&self, TInput) -> TResult;
// }

// impl<TSink, TInput, TResult> Dispatcher<TInput, TResult> for TSink
// where
//     TSink: Sink<TInput = TInput, TResult = TResult>,
// {
//     fn dispatch(&self, input: TInput) -> TResult {
//         self.send(input)
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

// pub trait Lift<TInput, TResult> {
//     fn lift<UInput, UResult>(&self, target: Sink<TInput=UInput, TResult=UResult>) -> Sink<TInput=TInput, TResult=UResult>
//     where
//         UInput: TResult,
//     {
//         target.map(self.send)
//     }
// }

// impl<'a, TSink, TInput, TResult> Dispatcher<TInput, TResult> for TSink
// where
//     TSink: SinkContainer<'a, TInput, TResult>,
// {
//     fn dispatch(&self, input: TInput) -> TResult {
//         self.sink().send(input)
//     }
// }

pub trait Source {
    type TOutput;

    fn next(&self) -> Self::TOutput;
}

pub trait Initializable: Default {
    type TState;

    fn init(state: Self::TState) -> Self {
        let mut default = Self::default();
        default.apply_state(state);
        default
    }

    fn apply_state(&mut self, state: Self::TState);
}
