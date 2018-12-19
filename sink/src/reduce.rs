use super::*;
use lib::core::marker::PhantomData;

/// Reduce dispatches Option Some(inputs) into a nested sink discarding None
#[derive(Clone, Debug)]
pub struct Reduce<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    target: TSink,
}

impl<TInput, TResult, TSink> Reduce<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    /// Build a new Reduce wrapper
    pub fn new(target: TSink) -> Self {
        Reduce {
            target,
        }
    }
}

impl<TInput, TResult, TSink> Sink for Reduce<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = Option<TInput>;
    type TResult = Option<TResult>;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        match input {
            None => None,
            Some (input) => Some (self.target.send(input))
        }
    }
}

impl<'a, TInput, TResult, TSink> Sink for &'a Reduce<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = Option<TInput>;
    type TResult = Option<TResult>;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        match input {
            None => None,
            Some (input) => Some (self.target.send(input))
        }
    }
}

/// The SinkReduce trait auto extends valid targets to wrap them in
/// a Reduce Sink
pub trait SinkReduce<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
    Self: Sink<TInput = TInput, TResult = TResult>,
{
    fn reduce(self) -> Reduce<TInput, TResult, TSink>;
}

impl<TInput, TResult, TSink> SinkReduce<TInput, TResult, TSink> for TSink
where
    Self: Sink<TInput = TInput, TResult = TResult>,
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    fn reduce(self) -> Reduce<TInput, TResult, TSink> {
        Reduce::new(self)
    }
}

#[cfg(test)]
mod should {
    use super::fnsink::FnSink;
    use super::sink::Sink;
    use super::*;
    use std::cell::{ RefCell };

    #[test]
    fn explicitly_construct() {
        let state = RefCell::new(Vec::new());
        let s = FnSink::new(|item: &'static str| {
            state.borrow_mut().push(item);
            item.len()
        });
        let s = Reduce::new(s);
        assert_eq!(Some(0), s.send(Some("")));
        assert_eq!(Some(9), s.send(Some("some text")));
        assert_eq!(None, s.send(None));
        assert_eq!(vec!["", "some text"], *state.borrow());
    }

    #[test]
    fn construct_through_the_reduce_function() {
        let state = RefCell::new(Vec::new());
        let s = FnSink::new(|item: &'static str| {
            state.borrow_mut().push(item);
            item.len()
        });
        let s = s.reduce();
        assert_eq!(Some(0), s.send(Some("")));
        assert_eq!(Some(9), s.send(Some("some text")));
        assert_eq!(None, s.send(None));
        assert_eq!(vec!["", "some text"], *state.borrow());
    }
}
