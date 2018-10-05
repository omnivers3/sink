use lib::core::marker::PhantomData;
use super::*;

/// Sink implementation which owns an internal state that is made available to
/// the provided handler when values are sent to it
pub struct StatefulSink<FHandler, TState, TInput, TResult>
where
    FHandler: Fn(&TState, TInput) -> TResult,
{
    state: TState,
    handler: FHandler,
    _input: PhantomData<TInput>,
}

impl<FHandler, TState, TInput, TResult> StatefulSink<FHandler, TState, TInput, TResult>
where
    FHandler: Fn(&TState, TInput) -> TResult,
{
    /// Builds a StatefulSink using the default for TState
    pub fn new(handler: FHandler) -> Self
    where
        TState: Default,
    {
        let default = TState::default();
        StatefulSink::with_state(default, handler)
    }

    /// Builds a StatefulSink using the TState provided
    pub fn with_state(state: TState, handler: FHandler) -> Self {
        StatefulSink {
            state: state,
            handler: handler,
            _input: PhantomData,
        }
    }
}

impl<FHandler, TState, TInput, TResult> ISink for StatefulSink<FHandler, TState, TInput, TResult>
where
    TState: Clone,
    FHandler: Fn(&TState, TInput) -> TResult,
{
    type TInput = TInput;
    type TResult = TResult;

    fn handle(&self, input: <Self as ISink>::TInput) -> <Self as ISink>::TResult {
        (self.handler)(&self.state, input)
    }
}

pub trait IntoStatefulSink
where
    Self: Sized,
{
    fn into_sink<TInput, TResult, FHandler: Fn(&Self, TInput) -> TResult>(
        self,
        handler: FHandler,
    ) -> StatefulSink<FHandler, Self, TInput, TResult> {
        StatefulSink::with_state(self, handler)
    }
}

impl<T> IntoStatefulSink for T where T: Clone {}

#[cfg(test)]
mod should {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn handle_single_item() {
        let s = StatefulSink::new(|_state: &(), _item| ());
        s.handle(());
    }

    #[test]
    fn handle_multiple_items() {
        let s = StatefulSink::new(|_state: &(), _item| ());
        s.handle(());
        s.handle(());
    }

    #[test]
    fn update_state_on_handle_given_mutable_type() {
        let initial = RefCell::new(10);
        let s = StatefulSink::with_state(&initial, |s, item| {
            let mut value = s.borrow_mut();
            *value += item;
            value.to_owned()
        });
        assert_eq!(20, s.handle(10));
        assert_eq!(40, s.handle(20));
    }

    #[test]
    fn update_state_on_handle_given_defaulted_mutable_type() {
        let s = StatefulSink::new(|s: &RefCell<u32>, item| {
            let mut value = s.borrow_mut();
            *value += item;
            value.to_owned()
        });
        assert_eq!(10, s.handle(10));
        assert_eq!(30, s.handle(20));
    }

    #[test]
    fn mutate_contained_state_after_into_sink() {
        let s = RefCell::<u32>::new(10).into_sink(|state, item| {
            let mut value = state.borrow_mut();
            *value += item;
            value.to_owned()
        });
        assert_eq!(20, s.handle(10));
        assert_eq!(40, s.handle(20));
    }
}
