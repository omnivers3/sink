use super::*;
use lib::core::marker::PhantomData;

/// Map transforms sink results.
pub struct ResultMap<FMap, TInput, TResult, UResult, TSink>
where
    FMap: Fn(TResult) -> UResult,
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    target: TSink,
    map: FMap,
    _u: PhantomData<UResult>,
}

impl<FMap, TInput, TResult, UResult, TSink> ResultMap<FMap, TInput, TResult, UResult, TSink>
where
    FMap: Fn(TResult) -> UResult,
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    /// Build a new SinkMap which uses the provided map to translate the incoming values
    /// into the target's expected type and an owned target allowing the caller to decide
    /// sharing rules
    pub fn new(target: TSink, map: FMap) -> Self
    where
        FMap: Fn(TResult) -> UResult,
    {
        ResultMap {
            target,
            map: map,
            _u: PhantomData,
        }
    }
}

impl<FMap, TInput, TResult, UResult, TSink> Sink for ResultMap<FMap, TInput, TResult, UResult, TSink>
where
    FMap: Fn(TResult) -> UResult,
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = TInput;
    type TResult = UResult;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        (self.map)(self.target.send(input))
    }
}

impl<'a, FMap, TInput, TResult, UResult, TSink> Sink for &'a ResultMap<FMap, TInput, TResult, UResult, TSink>
where
    FMap: Fn(TResult) -> UResult,
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = TInput;
    type TResult = UResult;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        (self.map)(self.target.send(input))
    }
}

/// The SinkResultMap trait describes the parameters necessary to link a target Sink
/// and a mapping function through a SinkResultMap, generaling it's constructor
pub trait SinkResultMap<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
    Self: Sink<TInput = TInput, TResult = TResult>,
{
    fn map_result<UResult, FMap>(self, map: FMap) -> ResultMap<FMap, TInput, TResult, UResult, TSink>
    where
        FMap: Fn(TResult) -> UResult;
}

impl<TInput, TResult, TSink> SinkResultMap<TInput, TResult, TSink> for TSink
where
    Self: Sink<TInput = TInput, TResult = TResult>,
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    fn map_result<UResult, FMap>(self, map: FMap) -> ResultMap<FMap, TInput, TResult, UResult, TSink>
    where
        FMap: Fn(TResult) -> UResult,
    {
        ResultMap::new(self, map)
    }
}

#[cfg(test)]
mod should {
    use super::fnsink::FnSink;
    use super::sink::Sink;
    use super::*;

    #[test]
    fn explicitly_construct() {
        let s = FnSink::new(|item: &'static str| item.len());
        let s = ResultMap::new(s, |item: usize| format!("len: {:?}", item));
        assert_eq!("len: 0", s.send(""));
        assert_eq!("len: 9", s.send("some text"));
    }

    #[test]
    fn construct_through_the_map_function() {
        let s = FnSink::new(|item: &'static str| item.len());
        let s = s.map_result(|item: usize| format!("len: {:?}", item));
        assert_eq!("len: 0", s.send(""));
        assert_eq!("len: 9", s.send("some text"));
    }
}
