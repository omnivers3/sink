use super::*;
use lib::core::marker::PhantomData;

/// Map transforms incomming data from source type to the type epxected by the wrapped ISink.
pub struct Map<FMap, TInput, UInput, TResult, TSink>
where
    FMap: Fn(UInput) -> TInput,
    TSink: ISink<TInput = TInput, TResult = TResult> + Sized,
{
    target: TSink,
    map: FMap,
    _uinput: PhantomData<UInput>,
}

impl<FMap, TInput, UInput, TResult, TSink> Map<FMap, TInput, UInput, TResult, TSink>
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
        Map {
            target,
            map: map,
            _uinput: PhantomData,
        }
    }
}

impl<FMap, TInput, UInput, TResult, TSink> ISink for Map<FMap, TInput, UInput, TResult, TSink>
where
    FMap: Fn(UInput) -> TInput,
    TSink: ISink<TInput = TInput, TResult = TResult>,
{
    type TInput = UInput;
    type TResult = TResult;

    fn send(&self, input: <Self as ISink>::TInput) -> <Self as ISink>::TResult {
        self.target.send((self.map)(input))
    }
}

/// The IMap trait describes the parameters necessary to link a target Sink
/// and a mapping function through a SinkMap, generaling it's constructor
pub trait IMap<TInput, TResult, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult>,
    Self: ISink<TInput = TInput, TResult = TResult>,
{
    fn map<UInput, FMap>(self, map: FMap) -> Map<FMap, TInput, UInput, TResult, TSink>
    where
        FMap: Fn(UInput) -> TInput;
}

impl<TInput, TResult, TSink> IMap<TInput, TResult, TSink> for TSink
where
    Self: ISink<TInput = TInput, TResult = TResult>,
    TSink: ISink<TInput = TInput, TResult = TResult>,
{
    fn map<UInput, FMap>(self, map: FMap) -> Map<FMap, TInput, UInput, TResult, TSink>
    where
        FMap: Fn(UInput) -> TInput,
    {
        Map::new(self, map)
    }
}

#[cfg(test)]
mod should {
    use super::sink::*;
    use super::*;

    #[test]
    fn explicitly_construct() {
        let s = Sink::new(|item| item);
        let s = Map::new(s, |item: &'static str| item.len());
        assert_eq!(0, s.send(""));
        assert_eq!(9, s.send("some text"));
    }

    #[test]
    fn construct_through_the_map_function() {
        let s = Sink::new(|item: usize| item);
        let s = s.map(|item: &'static str| item.len());
        assert_eq!(0, s.send(""));
        assert_eq!(9, s.send("some text"));
    }
}
