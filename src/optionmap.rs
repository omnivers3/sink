use super::*;
use lib::core::marker::PhantomData;

/// OptionMap transforms incomming option data from source type to the type epxected by the wrapped Sink.
pub struct OptionMap<FMap, TInput, UInput, TResult, TSink>
where
    FMap: Fn(UInput) -> TInput,
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    target: TSink,
    map: FMap,
    _u: PhantomData<UInput>,
}

impl<FMap, TInput, UInput, TResult, TSink> OptionMap<FMap, TInput, UInput, TResult, TSink>
where
    FMap: Fn(UInput) -> TInput,
    TSink: Sink<TInput = TInput, TResult = TResult> + Sized,
{
    /// Build a new SinkOptionMap which uses the provided map to translate the incoming values
    /// into the target's expected type and an owned target allowing the caller to decide
    /// sharing rules
    pub fn new(target: TSink, map: FMap) -> Self
    where
        FMap: Fn(UInput) -> TInput,
    {
        OptionMap {
            target,
            map: map,
            _u: PhantomData,
        }
    }
}

impl<FMap, TInput, UInput, TResult, TSink> Sink for OptionMap<FMap, TInput, UInput, TResult, TSink>
where
    FMap: Fn(UInput) -> TInput,
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = Option<UInput>;
    type TResult = Option<TResult>;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        match input {
            None => None,
            Some (input) => Some (self.target.send((self.map)(input)))
        }
    }
}

impl<'a, FMap, TInput, UInput, TResult, TSink> Sink for &'a OptionMap<FMap, TInput, UInput, TResult, TSink>
where
    FMap: Fn(UInput) -> TInput,
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    type TInput = Option<UInput>;
    type TResult = Option<TResult>;

    fn send(&self, input: <Self as Sink>::TInput) -> <Self as Sink>::TResult {
        match input {
            None => None,
            Some (input) => Some (self.target.send((self.map)(input)))
        }
    }
}

/// The SinkOptionMap trait describes the parameters necessary to link a target Sink
/// and a mapping function through a SinkOptionMap, generaling it's constructor
pub trait SinkOptionMap<TInput, TResult, TSink>
where
    TSink: Sink<TInput = TInput, TResult = TResult>,
    Self: Sink<TInput = TInput, TResult = TResult>,
{
    fn option_map<UInput, FMap>(self, map: FMap) -> OptionMap<FMap, TInput, UInput, TResult, TSink>
    where
        FMap: Fn(UInput) -> TInput;
}

impl<TInput, TResult, TSink> SinkOptionMap<TInput, TResult, TSink> for TSink
where
    Self: Sink<TInput = TInput, TResult = TResult>,
    TSink: Sink<TInput = TInput, TResult = TResult>,
{
    fn option_map<UInput, FMap>(self, map: FMap) -> OptionMap<FMap, TInput, UInput, TResult, TSink>
    where
        FMap: Fn(UInput) -> TInput,
    {
        OptionMap::new(self, map)
    }
}

#[cfg(test)]
mod should {
    use super::fnsink::FnSink;
    use super::sink::Sink;
    use super::*;
    use std::cell::{ RefCell };

    #[test]
    fn explicitly_construct() {
        let state = RefCell::new(Vec::new());
        let s = FnSink::new(|item| {
            state.borrow_mut().push(item);
            item
        });
        let state2 = RefCell::new(Vec::new());
        let s = OptionMap::new(s, |item: &'static str| {
            state2.borrow_mut().push(item);
            item.len()
        });
        assert_eq!(Some(0), s.send(Some("")));
        assert_eq!(Some(9), s.send(Some("some text")));
        assert_eq!(None, s.send(None));
        assert_eq!(vec![0, 9], *state.borrow());
        assert_eq!(vec!["", "some text"], *state2.borrow());
    }

    #[test]
    fn construct_through_the_OptionMap_function() {
        let state = RefCell::new(Vec::new());
        let s = FnSink::new(|item: usize| {
            state.borrow_mut().push(item);
            item
        });
        let state2 = RefCell::new(Vec::new());
        let s = s.option_map(|item: &'static str| {
            state2.borrow_mut().push(item);
            item.len()
        });
        assert_eq!(Some(0), s.send(Some("")));
        assert_eq!(Some(9), s.send(Some("some text")));
        assert_eq!(None, s.send(None));
        assert_eq!(vec![0, 9], *state.borrow());
        assert_eq!(vec!["", "some text"], *state2.borrow());
    }
}
