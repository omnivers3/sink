use lib::core::marker::PhantomData;

use super::*;

/// SinkMap transforms incomming data from source type to the type epxected by the wrapped ISink.
pub struct SinkMap<FMap, TSink, TInput, UInput, TResult, TError>
where
    FMap: Fn(UInput) -> TInput,
    TSink: ISink<TInput = TInput, TResult = TResult, TError = TError> + Sized,
{
    target: TSink,
    map: FMap,
    _uinput: PhantomData<UInput>,
}

impl<FMap, TSink, TInput, UInput, TResult, TError> SinkMap<FMap, TSink, TInput, UInput, TResult, TError>
where
    FMap: Fn(UInput) -> TInput,
    TSink: ISink<TInput = TInput, TResult = TResult, TError = TError> + Sized,
{
    /// Build a new SinkMap which uses the provided map to translate the incoming values
    /// into the target's expected type and an owned target allowing the caller to decide
    /// sharing rules
    pub fn new(target: TSink, map: FMap) -> Self
    where
        FMap: Fn(UInput) -> TInput,
    {
        SinkMap {
            target,
            map: map,
            _uinput: PhantomData,
        }
    }
}

impl<FMap, TSink, TInput, UInput, TResult, TError> ISink
    for SinkMap<FMap, TSink, TInput, UInput, TResult, TError>
where
    FMap: Fn(UInput) -> TInput,
    TSink: ISink<TInput = TInput, TResult = TResult, TError = TError>,
{
    type TInput = UInput;
    type TResult = TResult;
    type TError = TError;

    fn handle(
        &self,
        input: <Self as ISink>::TInput,
    ) -> Result<<Self as ISink>::TResult, <Self as ISink>::TError> {
        self.target.handle((self.map)(input))
    }
}

/// The ISinkMap trait describes the parameters necessary to link a target Sink
/// and a mapping function through a SinkMap, generaling it's constructor
pub trait ISinkMap<TInput, TResult, TError, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult, TError = TError>,
    Self: ISink<TInput = TInput, TResult = TResult, TError = TError>,
{
    fn map<UInput, FMap>(
        self,
        map: FMap,
    ) -> SinkMap<FMap, TSink, TInput, UInput, TResult, TError>
    where
        FMap: Fn(UInput) -> TInput;
}

impl<T, TInput, TResult, TError> ISinkMap<TInput, TResult, TError, T> for T
where
    Self: ISink<TInput = TInput, TResult = TResult, TError = TError>,
    T: ISink<TInput = TInput, TResult = TResult, TError = TError>,
{
    fn map<UInput, FMap>(
        self,
        map: FMap,
    ) -> SinkMap<FMap, T, TInput, UInput, TResult, TError>
    where
        FMap: Fn(UInput) -> TInput
    {
        SinkMap::new(self, map)
    }
}

#[cfg(test)]
mod sink_map_tests {
    use super::*;
    use super::sink::*;

    #[test]
    fn should_explicitly_construct_a_sinkmap() {
        let s = Sink::<_, _, _, ()>::new(|item| Ok(item));

        let sm = SinkMap::new(s, |item: &'static str| item.len());

        assert_eq!(Ok(0), sm.handle(""));
        assert_eq!(Ok(9), sm.handle("some text"));
    }

    #[test]
    fn should_construct_a_sinkmap_through_the_map_function() {
        let s = Sink::<_, _, _, ()>::new(|item| Ok(item)).map(|item: &'static str| item.len());

        assert_eq!(Ok(0), s.handle(""));
        assert_eq!(Ok(9), s.handle("some text"));
    }
}
