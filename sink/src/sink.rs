use lib::core::marker::PhantomData;

use super::*;

/// Sink is a simple struct which captures a provided handler function and routes
/// dispatched data into that handler
pub struct Sink<FHandler, TInput, TResult, TError>
where
    FHandler: Fn(TInput) -> Result<TResult, TError>,
{
    handler: FHandler,
    _input: PhantomData<TInput>,
}

impl<FHandler, TInput, TResult, TError> Sink<FHandler, TInput, TResult, TError>
where
    FHandler: Fn(TInput) -> Result<TResult, TError>,
{
    /// Builds a Sink using the provided handler
    pub fn new(handler: FHandler) -> Self
    where
        FHandler: Fn(TInput) -> Result<TResult, TError>,
    {
        Sink {
            handler,
            _input: PhantomData,
        }
    }
}

impl<FHandler, TInput, TResult, TError> ISink for Sink<FHandler, TInput, TResult, TError>
where
    FHandler: Fn(TInput) -> Result<TResult, TError>,
{
    type TInput = TInput;
    type TResult = TResult;
    type TError = TError;

    fn handle(
        &self,
        input: <Self as ISink>::TInput,
    ) -> Result<<Self as ISink>::TResult, <Self as ISink>::TError> {
        (self.handler)(input)
    }
}

#[cfg(test)]
mod sink_tests {
    use super::*;

    #[test]
    fn should_handle_single_unit_item_dispatched_to_sink() {
        let sink: Sink<_, _, _, ()> = Sink::new(|_item| Ok(()));

        sink.handle(()).unwrap();
    }

    #[test]
    fn should_handle_multiple_unit_items_dispatched_to_sink() {
        let sink: Sink<_, _, _, ()> = Sink::new(|_item| Ok(()));

        sink.handle(()).unwrap();
        sink.handle(()).unwrap();
    }

    #[test]
    fn should_handle_single_u32_item_dispatched_to_sink() {
        let sink: Sink<_, _, _, ()> = Sink::new(|item: u32| Ok(item));

        sink.handle(10).unwrap();
    }

    #[test]
    fn should_handle_multiple_u32_items_dispatched_to_sink() {
        let sink: Sink<_, _, _, ()> = Sink::new(|item: u32| Ok(item));

        assert_eq!(10, sink.handle(10).unwrap());
        assert_eq!(20, sink.handle(20).unwrap());
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestStruct {
        value: &'static str,
    }

    #[test]
    fn should_handle_single_struct_item_dispatched_to_sink() {
        let expected = TestStruct { value: "test" };

        let sink: Sink<_, _, _, ()> = Sink::new(|item| Ok(item));

        assert_eq!(expected.clone(), sink.handle(expected).unwrap());
    }

    #[test]
    fn should_handle_multiple_struct_items_dispatched_to_sink() {
        let expected1 = TestStruct { value: "test1" };
        let expected2 = TestStruct { value: "test2" };

        let sink: Sink<_, _, _, ()> = Sink::new(|item| Ok(item));

        assert_eq!(expected1.clone(), sink.handle(expected1).unwrap());
        assert_eq!(expected2.clone(), sink.handle(expected2).unwrap());
    }
}
