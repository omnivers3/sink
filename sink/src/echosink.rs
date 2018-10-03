use lib::core::marker::PhantomData;

use super::*;
use super::sink::{ Sink };

pub struct EchoSink<TInput> {
    _input: PhantomData<TInput>,
}

impl<TInput> EchoSink<TInput> {
    pub fn new() -> impl ISink<TInput=TInput, TResult=TInput> {
        Sink::new(|i| i)
    }
}

#[cfg(test)]
mod echosink_tests {
    use super::*;

    #[test]
    fn should_return_the_value_passed() {
        let sink = EchoSink::new();

        assert_eq!(10, sink.handle(10));
        // Cannot reuse the same echo sink for different value types...
        // assert_eq!("asdf", sink.handle("asdf"));
    }
}