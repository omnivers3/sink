use super::*;
use lib::core::marker::PhantomData;

pub struct Reflect<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    target: TSink,
    _input: PhantomData<TInput>,
}

impl<TInput, TResult, TSink> Reflect<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    pub fn new(target: TSink) -> Self {
        Reflect {
            target: target,
            _input: PhantomData,
        }
    }
}

impl<TInput, TResult, TSink> Sink for Reflect<TInput, TResult, TSink>
where
    TInput: Clone,
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = TInput;
    type TResult = (TInput, TResult);

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        (input.clone(), self.target.send(input))
    }
}

pub trait IReflect<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    fn reflect(self) -> Reflect<TInput, TResult, TSink>;
}

impl<TInput, TResult, TSink> IReflect<TInput, TResult, TSink> for TSink
where
    Self: Sink<TInput = TInput, TResult = TResult>,
{
    fn reflect(self) -> Reflect<TInput, TResult, TSink> {
        Reflect::new(self)
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
        let s = Reflect::new(s);
        assert_eq!(("", 0), s.send(""));
        assert_eq!(("some text", 9), s.send("some text"));
    }

    #[test]
    fn construct_through_the_map_function() {
        let s = FnSink::new(|i: u32| i * i);
        assert_eq!(100, s.send(10));
        let s = s.reflect();
        assert_eq!((10, 100), s.send(10));
    }
}
