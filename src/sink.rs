use super::*;

/// Sink is a simple struct which captures a provided handler function and routes
/// sent data into that handler
pub struct Sink<'a, TInput, TResult, TError> {
    handler: Box<Fn(TInput) -> Result<TResult, TError> + 'a>,
}

impl<'a, TInput, TResult, TError> Sink<'a, TInput, TResult, TError> {
    /// Builds a Sink using the provided handler
    pub fn new<F: 'a>(handler: F) -> Self
    where
        F: Fn(TInput) -> Result<TResult, TError> + 'a,
    {
        Sink {
            handler: Box::new(handler),
        }
    }
}

impl<'a, TInput, TResult, TError> ISink for Sink<'a, TInput, TResult, TError> {
    type TInput = TInput;
    type TResult = TResult;
    type TError = TError;

    fn send(
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
    fn should_send_single_item_to_sink() {
        let sink = Sink::<(), (), ()>::new(|_item| Ok(()));

        sink.send(()).unwrap();
    }

    #[test]
    fn should_send_multiple_items_to_sink() {
        let sink = Sink::<(), (), ()>::new(|_item| Ok(()));

        sink.send(()).unwrap();
        sink.send(()).unwrap();
    }
}
