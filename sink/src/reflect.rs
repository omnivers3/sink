use lib::core::marker::PhantomData;

use super::*;

pub struct Reflect<TInput, TResult, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult> + Sized,
{
    target: TSink,
    _input: PhantomData<TInput>,
}

impl<TInput, TResult, TSink> Reflect<TInput, TResult, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult> + Sized,
{
    pub fn new(target: TSink) -> Self {
        Reflect {
            target: target,
            _input: PhantomData,
        }
    }
}

impl<TInput, TResult, TSink> ISink for Reflect<TInput, TResult, TSink>
where
    TInput: Clone,
    TSink: ISink<TInput = TInput, TResult = TResult>,
{
    type TInput = TInput;
    type TResult = (TInput, TResult);

    fn handle(&self, input: <Self as ISink>::TInput) -> <Self as ISink>::TResult {
        (input.clone(), self.target.handle(input))
    }
}

pub trait IReflect<TInput, TResult, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult>,
{
    fn reflect(self) -> Reflect<TInput, TResult, TSink>;
}

impl<TInput, TResult, TSink> IReflect<TInput, TResult, TSink> for TSink
where
    Self: ISink<TInput = TInput, TResult = TResult>
{
    fn reflect(self) -> Reflect<TInput, TResult, TSink> {
        Reflect::new(self)
    }
}

#[cfg(test)]
mod reflect_tests {
    use super::sink::Sink;
    use super::*;

    #[test]
    fn should_explicitly_construct() {
        let s = Sink::new(|i: &str| i.len());
        let s = Reflect::new(s);

        assert_eq!(("", 0), s.handle(""));
        assert_eq!(("some text", 9), s.handle("some text"));
    }

    #[test]
    fn should_construct_through_the_map_function() {
        let sink = Sink::new(|i: u32| i * i);
        assert_eq!(100, sink.handle(10));
        let sink = sink.reflect();
        assert_eq!((10, 100), sink.handle(10));
    }
}
