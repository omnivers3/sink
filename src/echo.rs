use super::*;
use lib::core::marker::PhantomData;

pub struct EchoSink<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    _target: PhantomData<TSink>,
    _input: PhantomData<TInput>,
}

impl<TInput, TResult, TSink> EchoSink<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    pub fn new(_target: TSink) -> Self {
        EchoSink {
            _target: PhantomData,
            _input: PhantomData,
        }
    }
}

impl<TInput, TResult, TSink> Sink for EchoSink<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = TInput;
    type TResult = TInput;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TInput {
        input
    }
}

impl<'a, TInput, TResult, TSink> Sink for &'a EchoSink<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = TInput;
    type TResult = TInput;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TInput {
        input
    }
}

pub trait IEcho<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    fn echo(self) -> EchoSink<TInput, TResult, TSink>;
}

impl<TInput, TResult, TSink> IEcho<TInput, TResult, TSink> for TSink
where
    Self: Sink<TInput = TInput, TResult = TResult>,
{
    fn echo(self) -> EchoSink<TInput, TResult, TSink> {
        EchoSink::new(self)
    }
}

#[cfg(test)]
mod should {
    use super::fnsink::FnSink;
    use super::sink::Sink;
    use super::*;

    #[test]
    fn explicitly_construct() {
        let s = FnSink::new(|i: &str| i.len());
        let s = EchoSink::new(s);
        assert_eq!("", s.send(""));
        assert_eq!("some text", s.send("some text"));
    }

    #[test]
    fn construct_through_the_map_function() {
        let s = FnSink::new(|i: u32| i * i);
        assert_eq!(100, s.send(10));
        let s = s.echo();
        assert_eq!(10, s.send(10));
    }
}
