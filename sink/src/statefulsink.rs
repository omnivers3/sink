use super::*;

/// Sink implementation which owns an internal state that is made available to
/// the provided handler when values are sent to it
pub struct StatefulSink<'a, TState, TInput, TResult>
where
    TState: Clone,
{
    state: TState,
    handler: Box<Fn(TState, TInput) -> TResult + 'a>,
}

impl<'a, TState, TInput, TResult> StatefulSink<'a, TState, TInput, TResult>
where
    TState: Clone,
{
    /// Builds a StatefulSink using the default for TState
    pub fn new<F: 'a>(handler: F) -> Self
    where
        TState: Default,
        F: Fn(TState, TInput) -> TResult + 'a,
    {
        StatefulSink::with_state(TState::default(), handler)
    }

    /// Builds a StatefulSink using the TState provided
    pub fn with_state<F: 'a>(state: TState, handler: F) -> Self
    where
        F: Fn(TState, TInput) -> TResult + 'a,
    {
        StatefulSink {
            state: state,
            handler: Box::new(handler),
        }
    }
}

impl<'a, TState, TInput, TResult> ISink
    for StatefulSink<'a, TState, TInput, TResult>
where
    TState: Clone,
{
    type TInput = TInput;
    type TResult = TResult;

    fn handle(
        &self,
        input: <Self as ISink>::TInput,
    ) -> <Self as ISink>::TResult {
        (self.handler)(self.state.to_owned(), input)
    }
}

#[cfg(test)]
mod statefulsink_tests {
    use super::*;

    use std::cell::RefCell;

    #[test]
    fn should_handle_single_item_to_statefulsink() {
        let sink = StatefulSink::<(), _, _>::new(|_state, _item| ());

        sink.handle(());
    }

    #[test]
    fn should_handle_multiple_items_to_statefulsink() {
        let sink = StatefulSink::<(), _, _>::new(|_state, _item| ());

        sink.handle(());
        sink.handle(());
    }

    #[test]
    fn should_update_state_on_handle_given_mutable_type() {
        let initial = RefCell::new(10);

        let s =
            StatefulSink::with_state(&initial, |s, item| {
                let mut value = s.borrow_mut();
                *value += item;
                value.to_owned()
            });

        assert_eq!(20, s.handle(10));
        assert_eq!(40, s.handle(20));
    }
}