use lib::core::marker::PhantomData;

use super::*;

/// SinkMap transforms incomming data from source type to the type epxected by the wrapped ISink.
pub struct SinkMap<FMap, TSink, TInput, UInput, TResult>
where
    FMap: Fn(UInput) -> TInput,
    TSink: ISink<TInput = TInput, TResult = TResult> + Sized,
{
    target: TSink,
    map: FMap,
    _uinput: PhantomData<UInput>,
}

impl<FMap, TSink, TInput, UInput, TResult> SinkMap<FMap, TSink, TInput, UInput, TResult>
where
    FMap: Fn(UInput) -> TInput,
    TSink: ISink<TInput = TInput, TResult = TResult> + Sized,
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

impl<FMap, TSink, TInput, UInput, TResult> ISink
    for SinkMap<FMap, TSink, TInput, UInput, TResult>
where
    FMap: Fn(UInput) -> TInput,
    TSink: ISink<TInput = TInput, TResult = TResult>,
{
    type TInput = UInput;
    type TResult = TResult;

    fn handle(
        &self,
        input: <Self as ISink>::TInput,
    ) -> <Self as ISink>::TResult {
        self.target.handle((self.map)(input))
    }
}

/// The ISinkMap trait describes the parameters necessary to link a target Sink
/// and a mapping function through a SinkMap, generaling it's constructor
pub trait ISinkMap<TInput, TResult, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult>,
    Self: ISink<TInput = TInput, TResult = TResult>,
{
    fn map<UInput, FMap>(
        self,
        map: FMap,
    ) -> SinkMap<FMap, TSink, TInput, UInput, TResult>
    where
        FMap: Fn(UInput) -> TInput;
}

impl<T, TInput, TResult> ISinkMap<TInput, TResult, T> for T
where
    Self: ISink<TInput = TInput, TResult = TResult>,
    T: ISink<TInput = TInput, TResult = TResult>,
{
    fn map<UInput, FMap>(
        self,
        map: FMap,
    ) -> SinkMap<FMap, T, TInput, UInput, TResult>
    where
        FMap: Fn(UInput) -> TInput
    {
        SinkMap::new(self, map)
    }
}

#[cfg(test)]
mod sinkmap_tests {
    use super::*;
    use super::sink::*;

    #[test]
    fn should_explicitly_construct_a_sinkmap() {
        let s = Sink::new(|item| item);

        let sm = SinkMap::new(s, |item: &'static str| item.len());

        assert_eq!(0, sm.handle(""));
        assert_eq!(9, sm.handle("some text"));
    }

    #[test]
    fn should_construct_a_sinkmap_through_the_map_function() {
        let s = Sink::new(|item| item).map(|item: &'static str| item.len());

        assert_eq!(0, s.handle(""));
        assert_eq!(9, s.handle("some text"));
    }
}
