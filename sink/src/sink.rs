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

pub trait Dispatcher<TInput, TResult> {
    fn dispatch(&self, TInput) -> TResult;
}

impl<TSink, TInput, TResult> Dispatcher<TInput, TResult> for TSink
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    fn dispatch(&self, input: TInput) -> TResult {
        self.send(input)
    }
}

pub trait Source {
    type TOutput;

    fn next(&self) -> Self::TOutput;
}

pub trait Initializable: Default {
    type TState;

    fn init(state: Self::TState) -> Self {
        let mut default = Self::default();
        default.apply(state);
        default
    }

    fn apply(&mut self, state: Self::TState);
}

// pub trait IService {
//     type TInput;
//     type TOutput;
//     type THandle;

//     fn run(rx: Self::TInput, tx: Self::TOutput) -> Self::THandle;
// }

// pub trait IContext<TInput, TOutput, TOutputResult>
// where
//     Self: ISink<TInput=TOutput, TResult=TOutputResult>,
//     Self: ISource<TOutput=TInput>,
// {}

// pub trait ISystem {
//     type TInput;
//     type TOutput;
//     type TResult;
//     type THandle;

//     fn bind(ctx: impl IContext<Self::TInput, Self::TOutput, Self::TResult>) -> Self::THandle;
// }

// impl<TInput, TOutput, TOutputResult, T> IContext<TInput, TOutput, TOutputResult> for T
// where
//     T: ISource<TOutput=TInput>,
//     T: ISink<TInput=TOutput, TResult=TOutputResult>,
// {}
