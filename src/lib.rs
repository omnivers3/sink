mod sinkmap;

pub mod sink;
pub mod statefulsink;

pub use self::sinkmap::*;

/// The ISink trait aims to provide an abstraction for a thing which can be sent values
/// and return a Result indicating success / failure of receipt.
///
/// As a base primitive this should enable a message oriented variant of the
/// inbound params to the familiar imperitive Result 'and_then' composition pattern.
///
/// Immediately responding to send with Result<TResult, TError> enabling implementations
/// to encapsulate both sync and async processing with a sync response.
pub trait ISink {
    type TInput;
    type TResult;
    type TError;

    fn send(&self, input: Self::TInput) -> Result<Self::TResult, Self::TError>;
}