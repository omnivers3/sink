use super::*;
use lib::core::marker::PhantomData;

/// Map transforms incomming data from source type to the type epxected by the wrapped Sink.
pub struct Map<FMap, TInput, UInput, TResult, TSink>
where
    FMap: Fn(UInput) -> TInput,
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    target: TSink,
    map: FMap,
    _uinput: PhantomData<UInput>,
}

impl<FMap, TInput, UInput, TResult, TSink> Map<FMap, TInput, UInput, TResult, TSink>
where
    FMap: Fn(UInput) -> TInput,
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
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

impl<FMap, TInput, UInput, TResult, TSink> Sink for Map<FMap, TInput, UInput, TResult, TSink>
where
    FMap: Fn(UInput) -> TInput,
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = UInput;
    type TResult = TResult;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        self.target.send((self.map)(input))
    }
}

/// The SinkMap trait describes the parameters necessary to link a target Sink
/// and a mapping function through a SinkMap, generaling it's constructor
pub trait SinkMap<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
    Self: Sink<TInput = TInput, TResult = TResult>,
{
    fn map<UInput, FMap>(self, map: FMap) -> Map<FMap, TInput, UInput, TResult, TSink>
    where
        FMap: Fn(UInput) -> TInput;
}

impl<TInput, TResult, TSink> SinkMap<TInput, TResult, TSink> for TSink
where
    Self: Sink<TInput = TInput, TResult = TResult>,
    TSink: Sink<TInput = TInput, TResult = TResult>,
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
    use super::fnsink::FnSink;
    use super::sink::Sink;
    use super::*;

    #[test]
    fn explicitly_construct() {
        let s = FnSink::new(|item| item);
        let s = Map::new(s, |item: &'static str| item.len());
        assert_eq!(0, s.send(""));
        assert_eq!(9, s.send("some text"));
    }

    #[test]
    fn construct_through_the_map_function() {
        let s = FnSink::new(|item: usize| item);
        let s = s.map(|item: &'static str| item.len());
        assert_eq!(0, s.send(""));
        assert_eq!(9, s.send("some text"));
    }
}
