use lib::core::marker::PhantomData;
use sink::Sink;

/// FnSink is a simple struct which captures a provided handler function and routes
/// dispatched data into that handler
#[derive(Clone, Debug)]
pub struct FnSink<FHandler, TInput, TResult>
where
    FHandler: Fn(TInput) -> TResult,
{
    handler: FHandler,
    _input: PhantomData<TInput>,
}

impl<FHandler, TInput, TResult> FnSink<FHandler, TInput, TResult>
where
    FHandler: Fn(TInput) -> TResult,
{
    /// Builds a FnSink using the provided handler
    pub fn new(handler: FHandler) -> Self
    where
        FHandler: Fn(TInput) -> TResult,
    {
        FnSink {
            handler,
            _input: PhantomData,
        }
    }
}

impl<FHandler, TInput, TResult> Sink for FnSink<FHandler, TInput, TResult>
where
    FHandler: Fn(TInput) -> TResult,
{
    type TInput = TInput;
    type TResult = TResult;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        (self.handler)(input)
    }
}

impl<'a, FHandler, TInput, TResult> Sink for &'a FnSink<FHandler, TInput, TResult>
where
    FHandler: Fn(TInput) -> TResult,
{
    type TInput = TInput;
    type TResult = TResult;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        (self.handler)(input)
    }
}

impl<'a, TInput, TResult> Sink for &'a Fn(TInput) -> TResult {
    type TInput = TInput;
    type TResult = TResult;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        (self)(input)
    }
}

impl<FHandler, TInput, TResult> From<FHandler> for FnSink<FHandler, TInput, TResult>
where
    FHandler: Fn(TInput) -> TResult,
{
    fn from(handler: FHandler) -> Self {
        FnSink::new(handler)
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn handle_single_unit_item_dispatched_to_sink() {
        let s = FnSink::new(|_item| ());
        assert_eq!((), s.send(()));
    }

    #[test]
    fn handle_multiple_unit_items_dispatched_to_sink() {
        let s = FnSink::new(|_item| ());
        assert_eq!((), s.send(()));
        assert_eq!((), s.send(()));
    }

    #[test]
    fn echo_single_u32_item_dispatched_to_sink() {
        let s = FnSink::new(|item: u32| item);
        assert_eq!(10, s.send(10));
    }

    #[test]
    fn echo_multiple_u32_items_dispatched_to_sink() {
        let s = FnSink::new(|item: u32| item);
        assert_eq!(10, s.send(10));
        assert_eq!(20, s.send(20));
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestStruct {
        value: &'static str,
    }

    #[test]
    fn handle_single_struct_item_dispatched_to_sink() {
        let expected = TestStruct { value: "test" };
        let s = FnSink::new(|item| item);
        assert_eq!(expected.clone(), s.send(expected));
    }

    #[test]
    fn handle_multiple_struct_items_dispatched_to_sink() {
        let expected1 = TestStruct { value: "test1" };
        let expected2 = TestStruct { value: "test2" };
        let s = FnSink::new(|item| item);
        assert_eq!(expected1.clone(), s.send(expected1));
        assert_eq!(expected2.clone(), s.send(expected2));
    }
}
