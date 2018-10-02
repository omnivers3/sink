/// The ISink trait aims to provide an abstraction for a thing which can handle values
/// and return the result of each event's receipt along with a handle to, potentially,
/// observe outcome, status and/or value per the Sink type.
///
/// As a base primitive this should enable a message oriented variant of the inbound
/// params to the familiar imperitive Result 'and_then' composition pattern.
///
/// Immediately responding to handle with TResult enables implementations to represent
/// the potential for failure and encapsulate both sync and async processing.
pub trait ISink {
    type TInput;
    type TResult = ();

    /// `handle` accepts an item returning either a result, potentially unit, to the
    /// sender.  In practice the TResult can itself represent a more complex concept
    /// such as a Result<T,E>, a process handle or array index.
    fn handle(&self, event: Self::TInput) -> Self::TResult;
}
