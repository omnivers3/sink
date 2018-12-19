pub struct NullSink<T> {
    _t: PhantomData<T>
}

impl<T> NullSink<T> {
    pub fn new() -> Self {
        NullSink {
            _t: PhantomData,
        }
    }
}

impl<T> Sink for NullSink<T> {
    type TInput = T;
    type TResult = ();

    fn send(&self, input: Self::TInput) -> Self::TResult {
        ()
    }
}