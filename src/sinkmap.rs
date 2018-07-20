use super::*;

/// Transforms incomming data from source type to the type epxected by the wrapped ISink.
///
/// Explicitely building the SinkMap from ::new
///
pub struct SinkMap<'a, TInput, UInput, TResult, TError, TSink: Sized>
where
    TSink: ISink<TInput = TInput, TResult = TResult, TError = TError>,
{
    target: TSink,
    map: Box<Fn(UInput) -> TInput + 'a>,
}

impl<'a, TInput, UInput, TResult, TError, TSink> SinkMap<'a, TInput, UInput, TResult, TError, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult, TError = TError>,
{
    /// Build a new SinkMap which uses the provided map to translate the incoming values
    /// into the target's expected type and an owned target allowing the caller to decide
    /// sharing rules
    pub fn new<F: 'a>(target: TSink, map: F) -> Self
    where
        F: Fn(UInput) -> TInput + 'a,
    {
        SinkMap {
            target,
            map: Box::new(map),
        }
    }
}

impl<'a, TInput, UInput, TResult, TError, TSink> ISink
    for SinkMap<'a, TInput, UInput, TResult, TError, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult, TError = TError>,
{
    type TInput = UInput;
    type TResult = TResult;
    type TError = TError;

    fn send(
        &self,
        input: <Self as ISink>::TInput,
    ) -> Result<<Self as ISink>::TResult, <Self as ISink>::TError> {
        self.target.send((self.map)(input))
    }
}

/// The ISinkMap trait describes the parameters necessary to link a target Sink
/// and a mapping function through a SinkMap, generaling it's constructor
pub trait ISinkMap<'a, TInput, TResult, TError, TSink>
where
    TSink: ISink<TInput = TInput, TResult = TResult, TError = TError>,
    Self: ISink<TInput = TInput, TResult = TResult, TError = TError>,
{
    fn map<UInput, F: Fn(UInput) -> TInput + 'a>(
        self,
        map: F,
    ) -> SinkMap<'a, TInput, UInput, TResult, TError, TSink>;
}

impl<'a, T, TInput, TResult, TError> ISinkMap<'a, TInput, TResult, TError, T> for T
where
    Self: ISink<TInput = TInput, TResult = TResult, TError = TError>,
    T: ISink<TInput = TInput, TResult = TResult, TError = TError>,
{
    fn map<UInput, F: Fn(UInput) -> TInput + 'a>(
        self,
        map: F,
    ) -> SinkMap<'a, TInput, UInput, TResult, TError, T> {
        SinkMap::new(self, map)
    }
}

#[cfg(test)]
mod sink_map_tests {
    use super::*;
    use super::sink::*;

    #[test]
    fn should_explicitly_construct_a_sinkmap() {
        let s = Sink::<usize, usize, ()>::new(|item| Ok(item));

        let sm = SinkMap::new(s, |item: String| item.len());

        assert_eq!(Ok(0), sm.send("".to_owned()));
        assert_eq!(Ok(9), sm.send("some text".to_owned()));
    }

    #[test]
    fn should_construct_a_sinkmap_through_the_map_function() {
        let s = Sink::<usize, usize, ()>::new(|item| Ok(item)).map(|item: String| item.len());

        assert_eq!(Ok(0), s.send("".to_owned()));
        assert_eq!(Ok(9), s.send("some text".to_owned()));
    }
}
