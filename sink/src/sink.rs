use lib::core::marker::PhantomData;
use super::*;

/// Sink is a simple struct which captures a provided handler function and routes
/// dispatched data into that handler
pub struct Sink<FHandler, TInput, TResult>
where
    FHandler: Fn(TInput) -> TResult,
{
    handler: FHandler,
    _input: PhantomData<TInput>,
}

impl<FHandler, TInput, TResult> Sink<FHandler, TInput, TResult>
where
    FHandler: Fn(TInput) -> TResult,
{
    /// Builds a Sink using the provided handler
    pub fn new(handler: FHandler) -> Self
    where
        FHandler: Fn(TInput) -> TResult,
    {
        Sink {
            handler,
            _input: PhantomData,
        }
    }
}

impl<FHandler, TInput, TResult> ISink for Sink<FHandler, TInput, TResult>
where
    FHandler: Fn(TInput) -> TResult,
{
    type TInput = TInput;
    type TResult = TResult;

    fn handle(&self, input: <Self as ISink>::TInput) -> <Self as ISink>::TResult {
        (self.handler)(input)
    }
}

#[cfg(test)]
mod sink_tests {
    use super::*;

    #[test]
    fn should_handle_single_unit_item_dispatched_to_sink() {
        let s = Sink::new(|_item| ());
        assert_eq!((), s.handle(()));
    }

    #[test]
    fn should_handle_multiple_unit_items_dispatched_to_sink() {
        let s = Sink::new(|_item| ());
        assert_eq!((), s.handle(()));
        assert_eq!((), s.handle(()));
    }

    #[test]
    fn should_echo_single_u32_item_dispatched_to_sink() {
        let s = Sink::new(|item: u32| item);
        assert_eq!(10, s.handle(10));
    }

    #[test]
    fn should_echo_multiple_u32_items_dispatched_to_sink() {
        let s = Sink::new(|item: u32| item);
        assert_eq!(10, s.handle(10));
        assert_eq!(20, s.handle(20));
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestStruct {
        value: &'static str,
    }

    #[test]
    fn should_handle_single_struct_item_dispatched_to_sink() {
        let expected = TestStruct { value: "test" };
        let s = Sink::new(|item| item);
        assert_eq!(expected.clone(), s.handle(expected));
    }

    #[test]
    fn should_handle_multiple_struct_items_dispatched_to_sink() {
        let expected1 = TestStruct { value: "test1" };
        let expected2 = TestStruct { value: "test2" };
        let s = Sink::new(|item| item);
        assert_eq!(expected1.clone(), s.handle(expected1));
        assert_eq!(expected2.clone(), s.handle(expected2));
    }
}
