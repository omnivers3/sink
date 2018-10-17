use lib::core::marker::PhantomData;
use super::*;

pub struct Echo<TInput, TResult, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult> + Sized,
{
    _target: PhantomData<TSink>,
    _input: PhantomData<TInput>,
}

impl<TInput, TResult, TSink> Echo<TInput, TResult, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult> + Sized,
{
    pub fn new(_target: TSink) -> Self {
        Echo {
            _target: PhantomData,
            _input: PhantomData,
        }
    }
}

impl<TInput, TResult, TSink> ISink for Echo<TInput, TResult, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult>,
{
    type TInput = TInput;
    type TResult = TInput;

    fn send(&self, input: <Self as ISink>::TInput) -> <Self as ISink>::TInput {
        input
    }
}

pub trait IEcho<TInput, TResult, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult>,
{
    fn echo(self) -> Echo<TInput, TResult, TSink>;
}

impl<TInput, TResult, TSink> IEcho<TInput, TResult, TSink> for TSink
where
    Self: ISink<TInput = TInput, TResult = TResult>
{
    fn echo(self) -> Echo<TInput, TResult, TSink> {
        Echo::new(self)
    }
}

#[cfg(test)]
mod should {
    use super::sink::Sink;
    use super::*;

    #[test]
    fn explicitly_construct() {
        let s = Sink::new(|i: &str| i.len());
        let s = Echo::new(s);
        assert_eq!("", s.send(""));
        assert_eq!("some text", s.send("some text"));
    }

    #[test]
    fn construct_through_the_map_function() {
        let s = Sink::new(|i: u32| i * i);
        assert_eq!(100, s.send(10));
        let s = s.echo();
        assert_eq!(10, s.send(10));
    }
}
